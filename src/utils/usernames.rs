use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Username {
    TheXiao77,
    Javi,
    Victor,
    DarkTrainer,
    Awe,
    Toxic,
    Red,
    Dvdgg,
    Garfu,
    Sauturn,
    Mario,
    Jaime,
    Davas,
    DrDvd,
    McKay,
}

lazy_static! {
    static ref HANDLES: HashMap<Username, &'static str> = {
        let mut m = HashMap::new();
        m.insert(Username::TheXiao77, "@adrianv5x");
        m.insert(Username::Javi, "@LilNarwhal");
        m.insert(Username::Victor, "@SanZ97xX");
        m.insert(Username::DarkTrainer, "@DarkTrainer");
        m.insert(Username::Awe, "@JoseAwe");
        m.insert(Username::Toxic, "@txc450");
        m.insert(Username::Red, "@REDMSR");
        m.insert(Username::Dvdgg, "@Dvdgg");
        m.insert(Username::Garfu, "@Garfu01");
        m.insert(Username::Sauturn, "@Sauturn");
        m.insert(Username::Mario, "@CecilioGil");
        m.insert(Username::Jaime, "@jaimegsov");
        m.insert(Username::Davas, "@DavasJoe");
        m.insert(Username::DrDvd, "@THEDRDVD");
        m.insert(Username::McKay, "@DoctorMckay");
        m
    };
}

impl Username {
    #[must_use]
    pub fn telegram_handle(&self) -> &'static str {
        HANDLES[self]
    }
}

#[must_use]
pub fn get_telegram_handle(user: Username) -> &'static str {
    user.telegram_handle()
}
