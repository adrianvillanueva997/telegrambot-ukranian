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
    Andres,
}

impl Username {
    #[must_use]
    pub fn telegram_handle(&self) -> &'static str {
        match self {
            Self::TheXiao77 => "@adrianv5x",
            Self::Javi => "@LilNarwhal",
            Self::Victor => "@SanZ97xX",
            Self::DarkTrainer => "@DarkTrainer",
            Self::Awe => "@JoseAwe",
            Self::Toxic => "@txc450",
            Self::Red => "@REDMSR",
            Self::Dvdgg => "@Dvdgg",
            Self::Garfu => "@Garfu01",
            Self::Sauturn => "@Sauturn",
            Self::Mario => "@CecilioGil",
            Self::Jaime => "@jaimegsov",
            Self::Davas => "@DavasJoe",
            Self::DrDvd => "@THEDRDVD",
            Self::McKay => "@DoctorMckay",
            Self::Andres => "@AndresMorenoBlu",
        }
    }
}
