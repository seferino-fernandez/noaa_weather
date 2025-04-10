use serde::{Deserialize, Serialize};

/// NwsForecastOfficeId : Three-letter identifier for a NWS office.
/// Three-letter identifier for a NWS office.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum NwsForecastOfficeId {
    #[serde(rename = "AKQ")]
    Akq,
    #[serde(rename = "ALY")]
    Aly,
    #[serde(rename = "BGM")]
    Bgm,
    #[serde(rename = "BOX")]
    Box,
    #[serde(rename = "BTV")]
    Btv,
    #[serde(rename = "BUF")]
    Buf,
    #[serde(rename = "CAE")]
    Cae,
    #[serde(rename = "CAR")]
    Car,
    #[serde(rename = "CHS")]
    Chs,
    #[serde(rename = "CLE")]
    Cle,
    #[serde(rename = "CTP")]
    Ctp,
    #[serde(rename = "GSP")]
    Gsp,
    #[serde(rename = "GYX")]
    Gyx,
    #[serde(rename = "ILM")]
    Ilm,
    #[serde(rename = "ILN")]
    Iln,
    #[serde(rename = "LWX")]
    Lwx,
    #[serde(rename = "MHX")]
    Mhx,
    #[serde(rename = "OKX")]
    Okx,
    #[serde(rename = "PBZ")]
    Pbz,
    #[serde(rename = "PHI")]
    Phi,
    #[serde(rename = "RAH")]
    Rah,
    #[serde(rename = "RLX")]
    Rlx,
    #[serde(rename = "RNK")]
    Rnk,
    #[serde(rename = "ABQ")]
    Abq,
    #[serde(rename = "AMA")]
    Ama,
    #[serde(rename = "BMX")]
    Bmx,
    #[serde(rename = "BRO")]
    Bro,
    #[serde(rename = "CRP")]
    Crp,
    #[serde(rename = "EPZ")]
    Epz,
    #[serde(rename = "EWX")]
    Ewx,
    #[serde(rename = "FFC")]
    Ffc,
    #[serde(rename = "FWD")]
    Fwd,
    #[serde(rename = "HGX")]
    Hgx,
    #[serde(rename = "HUN")]
    Hun,
    #[serde(rename = "JAN")]
    Jan,
    #[serde(rename = "JAX")]
    Jax,
    #[serde(rename = "KEY")]
    Key,
    #[serde(rename = "LCH")]
    Lch,
    #[serde(rename = "LIX")]
    Lix,
    #[serde(rename = "LUB")]
    Lub,
    #[serde(rename = "LZK")]
    Lzk,
    #[serde(rename = "MAF")]
    Maf,
    #[serde(rename = "MEG")]
    Meg,
    #[serde(rename = "MFL")]
    Mfl,
    #[serde(rename = "MLB")]
    Mlb,
    #[serde(rename = "MOB")]
    Mob,
    #[serde(rename = "MRX")]
    Mrx,
    #[serde(rename = "OHX")]
    Ohx,
    #[serde(rename = "OUN")]
    Oun,
    #[serde(rename = "SHV")]
    Shv,
    #[serde(rename = "SJT")]
    Sjt,
    #[serde(rename = "SJU")]
    Sju,
    #[serde(rename = "TAE")]
    Tae,
    #[serde(rename = "TBW")]
    Tbw,
    #[serde(rename = "TSA")]
    Tsa,
    #[serde(rename = "ABR")]
    Abr,
    #[serde(rename = "APX")]
    Apx,
    #[serde(rename = "ARX")]
    Arx,
    #[serde(rename = "BIS")]
    Bis,
    #[serde(rename = "BOU")]
    Bou,
    #[serde(rename = "CYS")]
    Cys,
    #[serde(rename = "DDC")]
    Ddc,
    #[serde(rename = "DLH")]
    Dlh,
    #[serde(rename = "DMX")]
    Dmx,
    #[serde(rename = "DTX")]
    Dtx,
    #[serde(rename = "DVN")]
    Dvn,
    #[serde(rename = "EAX")]
    Eax,
    #[serde(rename = "FGF")]
    Fgf,
    #[serde(rename = "FSD")]
    Fsd,
    #[serde(rename = "GID")]
    Gid,
    #[serde(rename = "GJT")]
    Gjt,
    #[serde(rename = "GLD")]
    Gld,
    #[serde(rename = "GRB")]
    Grb,
    #[serde(rename = "GRR")]
    Grr,
    #[serde(rename = "ICT")]
    Ict,
    #[serde(rename = "ILX")]
    Ilx,
    #[serde(rename = "IND")]
    Ind,
    #[serde(rename = "IWX")]
    Iwx,
    #[serde(rename = "JKL")]
    Jkl,
    #[serde(rename = "LBF")]
    Lbf,
    #[serde(rename = "LMK")]
    Lmk,
    #[serde(rename = "LOT")]
    Lot,
    #[serde(rename = "LSX")]
    Lsx,
    #[serde(rename = "MKX")]
    Mkx,
    #[serde(rename = "MPX")]
    Mpx,
    #[serde(rename = "MQT")]
    Mqt,
    #[serde(rename = "OAX")]
    Oax,
    #[serde(rename = "PAH")]
    Pah,
    #[serde(rename = "PUB")]
    Pub,
    #[serde(rename = "RIW")]
    Riw,
    #[serde(rename = "SGF")]
    Sgf,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "UNR")]
    Unr,
    #[serde(rename = "BOI")]
    Boi,
    #[serde(rename = "BYZ")]
    Byz,
    #[serde(rename = "EKA")]
    Eka,
    #[serde(rename = "FGZ")]
    Fgz,
    #[serde(rename = "GGW")]
    Ggw,
    #[serde(rename = "HNX")]
    Hnx,
    #[serde(rename = "LKN")]
    Lkn,
    #[serde(rename = "LOX")]
    Lox,
    #[serde(rename = "MFR")]
    Mfr,
    #[serde(rename = "MSO")]
    Mso,
    #[serde(rename = "MTR")]
    Mtr,
    #[serde(rename = "OTX")]
    Otx,
    #[serde(rename = "PDT")]
    Pdt,
    #[serde(rename = "PIH")]
    Pih,
    #[serde(rename = "PQR")]
    Pqr,
    #[serde(rename = "PSR")]
    Psr,
    #[serde(rename = "REV")]
    Rev,
    #[serde(rename = "SEW")]
    Sew,
    #[serde(rename = "SGX")]
    Sgx,
    #[serde(rename = "SLC")]
    Slc,
    #[serde(rename = "STO")]
    Sto,
    #[serde(rename = "TFX")]
    Tfx,
    #[serde(rename = "TWC")]
    Twc,
    #[serde(rename = "VEF")]
    Vef,
    #[serde(rename = "AER")]
    Aer,
    #[serde(rename = "AFC")]
    Afc,
    #[serde(rename = "AFG")]
    Afg,
    #[serde(rename = "AJK")]
    Ajk,
    #[serde(rename = "ALU")]
    Alu,
    #[serde(rename = "GUM")]
    Gum,
    #[serde(rename = "HPA")]
    Hpa,
    #[serde(rename = "HFO")]
    Hfo,
    #[serde(rename = "PPG")]
    Ppg,
    #[serde(rename = "STU")]
    Stu,
    #[serde(rename = "NH1")]
    Nh1,
    #[serde(rename = "NH2")]
    Nh2,
    #[serde(rename = "ONA")]
    Ona,
    #[serde(rename = "ONP")]
    Onp,
    #[serde(rename = "PQE")]
    Pqe,
    #[serde(rename = "PQW")]
    Pqw,
}

impl std::fmt::Display for NwsForecastOfficeId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Akq => write!(f, "AKQ"),
            Self::Aly => write!(f, "ALY"),
            Self::Bgm => write!(f, "BGM"),
            Self::Box => write!(f, "BOX"),
            Self::Btv => write!(f, "BTV"),
            Self::Buf => write!(f, "BUF"),
            Self::Cae => write!(f, "CAE"),
            Self::Car => write!(f, "CAR"),
            Self::Chs => write!(f, "CHS"),
            Self::Cle => write!(f, "CLE"),
            Self::Ctp => write!(f, "CTP"),
            Self::Gsp => write!(f, "GSP"),
            Self::Gyx => write!(f, "GYX"),
            Self::Ilm => write!(f, "ILM"),
            Self::Iln => write!(f, "ILN"),
            Self::Lwx => write!(f, "LWX"),
            Self::Mhx => write!(f, "MHX"),
            Self::Okx => write!(f, "OKX"),
            Self::Pbz => write!(f, "PBZ"),
            Self::Phi => write!(f, "PHI"),
            Self::Rah => write!(f, "RAH"),
            Self::Rlx => write!(f, "RLX"),
            Self::Rnk => write!(f, "RNK"),
            Self::Abq => write!(f, "ABQ"),
            Self::Ama => write!(f, "AMA"),
            Self::Bmx => write!(f, "BMX"),
            Self::Bro => write!(f, "BRO"),
            Self::Crp => write!(f, "CRP"),
            Self::Epz => write!(f, "EPZ"),
            Self::Ewx => write!(f, "EWX"),
            Self::Ffc => write!(f, "FFC"),
            Self::Fwd => write!(f, "FWD"),
            Self::Hgx => write!(f, "HGX"),
            Self::Hun => write!(f, "HUN"),
            Self::Jan => write!(f, "JAN"),
            Self::Jax => write!(f, "JAX"),
            Self::Key => write!(f, "KEY"),
            Self::Lch => write!(f, "LCH"),
            Self::Lix => write!(f, "LIX"),
            Self::Lub => write!(f, "LUB"),
            Self::Lzk => write!(f, "LZK"),
            Self::Maf => write!(f, "MAF"),
            Self::Meg => write!(f, "MEG"),
            Self::Mfl => write!(f, "MFL"),
            Self::Mlb => write!(f, "MLB"),
            Self::Mob => write!(f, "MOB"),
            Self::Mrx => write!(f, "MRX"),
            Self::Ohx => write!(f, "OHX"),
            Self::Oun => write!(f, "OUN"),
            Self::Shv => write!(f, "SHV"),
            Self::Sjt => write!(f, "SJT"),
            Self::Sju => write!(f, "SJU"),
            Self::Tae => write!(f, "TAE"),
            Self::Tbw => write!(f, "TBW"),
            Self::Tsa => write!(f, "TSA"),
            Self::Abr => write!(f, "ABR"),
            Self::Apx => write!(f, "APX"),
            Self::Arx => write!(f, "ARX"),
            Self::Bis => write!(f, "BIS"),
            Self::Bou => write!(f, "BOU"),
            Self::Cys => write!(f, "CYS"),
            Self::Ddc => write!(f, "DDC"),
            Self::Dlh => write!(f, "DLH"),
            Self::Dmx => write!(f, "DMX"),
            Self::Dtx => write!(f, "DTX"),
            Self::Dvn => write!(f, "DVN"),
            Self::Eax => write!(f, "EAX"),
            Self::Fgf => write!(f, "FGF"),
            Self::Fsd => write!(f, "FSD"),
            Self::Gid => write!(f, "GID"),
            Self::Gjt => write!(f, "GJT"),
            Self::Gld => write!(f, "GLD"),
            Self::Grb => write!(f, "GRB"),
            Self::Grr => write!(f, "GRR"),
            Self::Ict => write!(f, "ICT"),
            Self::Ilx => write!(f, "ILX"),
            Self::Ind => write!(f, "IND"),
            Self::Iwx => write!(f, "IWX"),
            Self::Jkl => write!(f, "JKL"),
            Self::Lbf => write!(f, "LBF"),
            Self::Lmk => write!(f, "LMK"),
            Self::Lot => write!(f, "LOT"),
            Self::Lsx => write!(f, "LSX"),
            Self::Mkx => write!(f, "MKX"),
            Self::Mpx => write!(f, "MPX"),
            Self::Mqt => write!(f, "MQT"),
            Self::Oax => write!(f, "OAX"),
            Self::Pah => write!(f, "PAH"),
            Self::Pub => write!(f, "PUB"),
            Self::Riw => write!(f, "RIW"),
            Self::Sgf => write!(f, "SGF"),
            Self::Top => write!(f, "TOP"),
            Self::Unr => write!(f, "UNR"),
            Self::Boi => write!(f, "BOI"),
            Self::Byz => write!(f, "BYZ"),
            Self::Eka => write!(f, "EKA"),
            Self::Fgz => write!(f, "FGZ"),
            Self::Ggw => write!(f, "GGW"),
            Self::Hnx => write!(f, "HNX"),
            Self::Lkn => write!(f, "LKN"),
            Self::Lox => write!(f, "LOX"),
            Self::Mfr => write!(f, "MFR"),
            Self::Mso => write!(f, "MSO"),
            Self::Mtr => write!(f, "MTR"),
            Self::Otx => write!(f, "OTX"),
            Self::Pdt => write!(f, "PDT"),
            Self::Pih => write!(f, "PIH"),
            Self::Pqr => write!(f, "PQR"),
            Self::Psr => write!(f, "PSR"),
            Self::Rev => write!(f, "REV"),
            Self::Sew => write!(f, "SEW"),
            Self::Sgx => write!(f, "SGX"),
            Self::Slc => write!(f, "SLC"),
            Self::Sto => write!(f, "STO"),
            Self::Tfx => write!(f, "TFX"),
            Self::Twc => write!(f, "TWC"),
            Self::Vef => write!(f, "VEF"),
            Self::Aer => write!(f, "AER"),
            Self::Afc => write!(f, "AFC"),
            Self::Afg => write!(f, "AFG"),
            Self::Ajk => write!(f, "AJK"),
            Self::Alu => write!(f, "ALU"),
            Self::Gum => write!(f, "GUM"),
            Self::Hpa => write!(f, "HPA"),
            Self::Hfo => write!(f, "HFO"),
            Self::Ppg => write!(f, "PPG"),
            Self::Stu => write!(f, "STU"),
            Self::Nh1 => write!(f, "NH1"),
            Self::Nh2 => write!(f, "NH2"),
            Self::Ona => write!(f, "ONA"),
            Self::Onp => write!(f, "ONP"),
            Self::Pqe => write!(f, "PQE"),
            Self::Pqw => write!(f, "PQW"),
        }
    }
}

impl Default for NwsForecastOfficeId {
    fn default() -> NwsForecastOfficeId {
        Self::Akq
    }
}
