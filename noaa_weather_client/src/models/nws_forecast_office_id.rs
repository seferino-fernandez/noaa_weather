use std::fmt;
use std::str::FromStr;

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

impl fmt::Display for NwsForecastOfficeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseNwsForecastOfficeIdError {
    invalid_value: String,
}

impl fmt::Display for ParseNwsForecastOfficeIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid NWS forecast office ID: {}", self.invalid_value)
    }
}

impl std::error::Error for ParseNwsForecastOfficeIdError {}

impl FromStr for NwsForecastOfficeId {
    type Err = ParseNwsForecastOfficeIdError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let lower_string = string.to_lowercase();
        match lower_string.as_str() {
            "akq" => Ok(Self::Akq),
            "aly" => Ok(Self::Aly),
            "bgm" => Ok(Self::Bgm),
            "box" => Ok(Self::Box),
            "btv" => Ok(Self::Btv),
            "buf" => Ok(Self::Buf),
            "cae" => Ok(Self::Cae),
            "car" => Ok(Self::Car),
            "chs" => Ok(Self::Chs),
            "cle" => Ok(Self::Cle),
            "ctp" => Ok(Self::Ctp),
            "gsp" => Ok(Self::Gsp),
            "gyx" => Ok(Self::Gyx),
            "ilm" => Ok(Self::Ilm),
            "iln" => Ok(Self::Iln),
            "lwx" => Ok(Self::Lwx),
            "mhx" => Ok(Self::Mhx),
            "okx" => Ok(Self::Okx),
            "pbz" => Ok(Self::Pbz),
            "phi" => Ok(Self::Phi),
            "rah" => Ok(Self::Rah),
            "rlx" => Ok(Self::Rlx),
            "rnk" => Ok(Self::Rnk),
            "abq" => Ok(Self::Abq),
            "ama" => Ok(Self::Ama),
            "bmx" => Ok(Self::Bmx),
            "bro" => Ok(Self::Bro),
            "crp" => Ok(Self::Crp),
            "epz" => Ok(Self::Epz),
            "ewx" => Ok(Self::Ewx),
            "ffc" => Ok(Self::Ffc),
            "fwd" => Ok(Self::Fwd),
            "hgx" => Ok(Self::Hgx),
            "hun" => Ok(Self::Hun),
            "jan" => Ok(Self::Jan),
            "jax" => Ok(Self::Jax),
            "key" => Ok(Self::Key),
            "lch" => Ok(Self::Lch),
            "lix" => Ok(Self::Lix),
            "lub" => Ok(Self::Lub),
            "lzk" => Ok(Self::Lzk),
            "maf" => Ok(Self::Maf),
            "meg" => Ok(Self::Meg),
            "mfl" => Ok(Self::Mfl),
            "mlb" => Ok(Self::Mlb),
            "mob" => Ok(Self::Mob),
            "mrx" => Ok(Self::Mrx),
            "ohx" => Ok(Self::Ohx),
            "oun" => Ok(Self::Oun),
            "shv" => Ok(Self::Shv),
            "sjt" => Ok(Self::Sjt),
            "sju" => Ok(Self::Sju),
            "tae" => Ok(Self::Tae),
            "tbw" => Ok(Self::Tbw),
            "tsa" => Ok(Self::Tsa),
            "abr" => Ok(Self::Abr),
            "apx" => Ok(Self::Apx),
            "arx" => Ok(Self::Arx),
            "bis" => Ok(Self::Bis),
            "bou" => Ok(Self::Bou),
            "cys" => Ok(Self::Cys),
            "ddc" => Ok(Self::Ddc),
            "dlh" => Ok(Self::Dlh),
            "dmx" => Ok(Self::Dmx),
            "dtx" => Ok(Self::Dtx),
            "dvn" => Ok(Self::Dvn),
            "eax" => Ok(Self::Eax),
            "fgf" => Ok(Self::Fgf),
            "fsd" => Ok(Self::Fsd),
            "gid" => Ok(Self::Gid),
            "gjt" => Ok(Self::Gjt),
            "gld" => Ok(Self::Gld),
            "grb" => Ok(Self::Grb),
            "grr" => Ok(Self::Grr),
            "ict" => Ok(Self::Ict),
            "ilx" => Ok(Self::Ilx),
            "ind" => Ok(Self::Ind),
            "iwx" => Ok(Self::Iwx),
            "jkl" => Ok(Self::Jkl),
            "lbf" => Ok(Self::Lbf),
            "lmk" => Ok(Self::Lmk),
            "lot" => Ok(Self::Lot),
            "lsx" => Ok(Self::Lsx),
            "mkx" => Ok(Self::Mkx),
            "mpx" => Ok(Self::Mpx),
            "mqt" => Ok(Self::Mqt),
            "oax" => Ok(Self::Oax),
            "pah" => Ok(Self::Pah),
            "pub" => Ok(Self::Pub),
            "riw" => Ok(Self::Riw),
            "sgf" => Ok(Self::Sgf),
            "top" => Ok(Self::Top),
            "unr" => Ok(Self::Unr),
            "boi" => Ok(Self::Boi),
            "byz" => Ok(Self::Byz),
            "eka" => Ok(Self::Eka),
            "fgz" => Ok(Self::Fgz),
            "ggw" => Ok(Self::Ggw),
            "hnx" => Ok(Self::Hnx),
            "lkn" => Ok(Self::Lkn),
            "lox" => Ok(Self::Lox),
            "mfr" => Ok(Self::Mfr),
            "mso" => Ok(Self::Mso),
            "mtr" => Ok(Self::Mtr),
            "otx" => Ok(Self::Otx),
            "pdt" => Ok(Self::Pdt),
            "pih" => Ok(Self::Pih),
            "pqr" => Ok(Self::Pqr),
            "psr" => Ok(Self::Psr),
            "rev" => Ok(Self::Rev),
            "sew" => Ok(Self::Sew),
            "sgx" => Ok(Self::Sgx),
            "slc" => Ok(Self::Slc),
            "sto" => Ok(Self::Sto),
            "tfx" => Ok(Self::Tfx),
            "twc" => Ok(Self::Twc),
            "vef" => Ok(Self::Vef),
            "aer" => Ok(Self::Aer),
            "afc" => Ok(Self::Afc),
            "afg" => Ok(Self::Afg),
            "ajk" => Ok(Self::Ajk),
            "alu" => Ok(Self::Alu),
            "gum" => Ok(Self::Gum),
            "hpa" => Ok(Self::Hpa),
            "hfo" => Ok(Self::Hfo),
            "ppg" => Ok(Self::Ppg),
            "stu" => Ok(Self::Stu),
            "nh1" => Ok(Self::Nh1),
            "nh2" => Ok(Self::Nh2),
            "ona" => Ok(Self::Ona),
            "onp" => Ok(Self::Onp),
            "pqe" => Ok(Self::Pqe),
            "pqw" => Ok(Self::Pqw),
            _ => Err(ParseNwsForecastOfficeIdError {
                invalid_value: string.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(NwsForecastOfficeId::Akq.to_string(), "AKQ");
        assert_eq!(NwsForecastOfficeId::Pqw.to_string(), "PQW");
        assert_eq!(NwsForecastOfficeId::Maf.to_string(), "MAF");
        assert_eq!(NwsForecastOfficeId::Sew.to_string(), "SEW");
    }

    #[test]
    fn test_from_str_ok() {
        assert_eq!(
            "AKQ".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Akq)
        );
        assert_eq!(
            "akq".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Akq)
        );
        assert_eq!(
            "PQW".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Pqw)
        );
        assert_eq!(
            "pqw".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Pqw)
        );
        assert_eq!(
            "MAF".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Maf)
        );
        assert_eq!(
            "maf".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Maf)
        );
        assert_eq!(
            "SEW".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Sew)
        );
        assert_eq!(
            "sew".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Sew)
        );
        assert_eq!(
            "SGF".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Sgf)
        );
        assert_eq!(
            "sgf".parse::<NwsForecastOfficeId>(),
            Ok(NwsForecastOfficeId::Sgf)
        );
    }

    #[test]
    fn test_from_str_err() {
        assert_eq!(
            "INVALID".parse::<NwsForecastOfficeId>(),
            Err(ParseNwsForecastOfficeIdError {
                invalid_value: "INVALID".to_string()
            })
        );
        assert_eq!(
            "ak".parse::<NwsForecastOfficeId>(),
            Err(ParseNwsForecastOfficeIdError {
                invalid_value: "ak".to_string()
            })
        );
        assert_eq!(
            "".parse::<NwsForecastOfficeId>(),
            Err(ParseNwsForecastOfficeIdError {
                invalid_value: String::new()
            })
        );
    }
}
