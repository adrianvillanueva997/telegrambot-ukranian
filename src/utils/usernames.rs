use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct User {
    pub name: &'static str,
    pub telegram_handle: &'static str,
}

lazy_static! {
    pub static ref USERS: HashMap<&'static str, User> = {
        let mut m = HashMap::new();
        m.insert(
            "THEXIAO77",
            User {
                name: "THEXIAO77",
                telegram_handle: "@adrianv5x",
            },
        );
        m.insert(
            "JAVI",
            User {
                name: "JAVI",
                telegram_handle: "@LilNarwhal",
            },
        );
        m.insert(
            "VICTOR",
            User {
                name: "VICTOR",
                telegram_handle: "@SanZ97xX",
            },
        );
        m.insert(
            "DARKTRAINER",
            User {
                name: "DARKTRAINER",
                telegram_handle: "@DarkTrainer",
            },
        );
        m.insert(
            "AWE",
            User {
                name: "AWE",
                telegram_handle: "@JoseAwe",
            },
        );
        m.insert(
            "TOXIC",
            User {
                name: "TOXIC",
                telegram_handle: "@txc450",
            },
        );
        m.insert(
            "RED",
            User {
                name: "RED",
                telegram_handle: "@REDMSR",
            },
        );
        m.insert(
            "DVDGG",
            User {
                name: "DVDGG",
                telegram_handle: "@Dvdgg",
            },
        );
        m.insert(
            "GARFU",
            User {
                name: "GARFU",
                telegram_handle: "@Garfu01",
            },
        );
        m.insert(
            "SAUTURN",
            User {
                name: "SAUTURN",
                telegram_handle: "@Sauturn",
            },
        );
        m.insert(
            "MARIO",
            User {
                name: "MARIO",
                telegram_handle: "@CecilioGil",
            },
        );
        m.insert(
            "JAIME",
            User {
                name: "JAIME",
                telegram_handle: "@jaimegsov",
            },
        );
        m.insert(
            "DAVAS",
            User {
                name: "DAVAS",
                telegram_handle: "@DavasJoe",
            },
        );
        m.insert(
            "DRDVD",
            User {
                name: "DRDVD",
                telegram_handle: "@THEDRDVD",
            },
        );
        m.insert(
            "MCKAY",
            User {
                name: "MCKAY",
                telegram_handle: "@DoctorMckay",
            },
        );
        m
    };
}

/// Get the telegram handle of a user by their name (case-sensitive)
// Helper functions
#[must_use]
pub fn get_telegram_handle(name: &str) -> Option<&'static str> {
    USERS.get(name).map(|user| user.telegram_handle)
}
