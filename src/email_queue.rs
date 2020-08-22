use crate::config::CONFIG;
use crate::connection::ConnectionType;
use crate::schema::email_queue;
use crate::schema::email_queue::dsl::*;
use diesel::prelude::*;
use diesel::{RunQueryDsl, ExpressionMethods};
use chrono::{NaiveDateTime, Duration, Utc};
use lettre::{SmtpTransport, ClientTlsParameters, SmtpClient, ClientSecurity, SendableEmail, Transport};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre_email::EmailBuilder;
use lettre::smtp::ConnectionReuseParameters;
use native_tls::{Protocol, TlsConnector};

const STATUS_PENDING : i8 = 0;
const STATUS_FAILED : i8 = 1;
const STATUS_SENT : i8 = 8;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, QueryableByName, Queryable, Identifiable)]
#[primary_key(_id)]
#[table_name = "email_queue"]
pub struct EmailQueue {
    pub _id: i64,
    pub to_address: String,
    pub subject: String,
    pub mail_body: String,
    pub from: Option<String>,
    pub send_count: i32
}
#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "email_queue"]
pub struct QueueStatusUpdate {
    pub _id: i64,
    pub status: i8,
    pub sent_time: Option<NaiveDateTime>,
    pub send_count: i32,
    pub last_error: Option<String>
}

/// Create a new user
pub fn check_queue(connection: &ConnectionType){
    let current = Utc::now().naive_local();
    let from_date = current.checked_sub_signed(Duration::seconds(CONFIG.send_duration)).unwrap();
    let unsent_mails = email_queue
        .select((_id,to_address,subject,mail_body,from,send_count))
        .filter(send_time.between(from_date, current))
        .filter(status.eq_any(&[STATUS_PENDING, STATUS_FAILED]))
        .order(from.asc())
        .load::<EmailQueue>(connection);
    match unsent_mails {
        Ok(emails) => {
            if !emails.is_empty() {
                let mut from_email: (&str, &str, &str) = get_from_email(&emails[0].from.as_ref().unwrap().as_str());
                let mut prev_from = &emails[0].from;
                let mut mailer: SmtpTransport = build_mailer(from_email);
                for mail in &emails {
                    if prev_from != &mail.from {
                        prev_from = &mail.from;
                        from_email = get_from_email(mail.from.as_ref().unwrap().as_str());
                        mailer = build_mailer(from_email);
                    }
                    send_email(connection, mail, &mut mailer, from_email);
                }
            }
        },
        Err(_e) => {}
    }
}

fn get_from_email(from_email: &str) -> (&str, &str, &str) {
    match from_email {
        "INFO" | "NOTICE" => {
            (CONFIG.info_email.as_str(), CONFIG.info_name.as_str(), CONFIG.info_pass.as_str())
        }
        _ => {
            (from_email, "", "")
        }
    }
}

fn send_email(connection: &ConnectionType, record: &EmailQueue, mailer: &mut SmtpTransport, from_email: (&str, &str, &str)){

    let email: SendableEmail = EmailBuilder::new()
        .to(record.to_address.as_str())
        .from((from_email.0, from_email.1))
        .subject(record.subject.as_str())
        .html(record.mail_body.as_str())
        .build()
        .unwrap().into();

    let log : QueueStatusUpdate;
    match mailer.send(email) {
        Ok(_) => {
            log = QueueStatusUpdate{
                _id: record._id,
                status: STATUS_SENT,
                sent_time: Some(Utc::now().naive_local()),
                send_count: record.send_count+1,
                last_error: None
            };
        },
        Err(e) => {
            log = QueueStatusUpdate{
                _id: record._id,
                status: STATUS_FAILED,
                sent_time: None,
                send_count: record.send_count+1,
                last_error: Some(e.to_string())
            };
        },
    }
    diesel::update(email_queue)
        .filter(_id.eq(log._id.clone()))
        .set(log)
        .execute(connection);

}

fn build_mailer(from_email: (&str, &str, &str)) -> SmtpTransport {
    let mut tls_builder = TlsConnector::builder();
    let server_addr : Vec<&str> = CONFIG.mail_server.split(':').collect();
    tls_builder.min_protocol_version(Some(Protocol::Tlsv12));
    let tls_parameters =
        ClientTlsParameters::new(
            server_addr[0].to_string(),
            tls_builder.build().unwrap()
        );
    let mut port : u16 = 465;
    match server_addr[1].to_string().parse::<u16>() {
        Ok(p) => port = p,
        Err(_e) => {}
    }
    let mailer = SmtpClient::new(
        (server_addr[0], port), ClientSecurity::Wrapper(tls_parameters)
    ).unwrap()
        .authentication_mechanism(Mechanism::Login)
        .credentials(Credentials::new(
            from_email.0.to_string(), from_email.2.to_string()
        ))
        .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
        // Enable SMTPUTF8 if the server supports it
        .smtp_utf8(true)
        .transport();
    return mailer;
}