use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Identifies a forecast office or forecast grid domain accepted by NOAA.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema), schemars(inline))]
pub enum NwsForecastOfficeId {
    /// The `AKQ` identifier for the Wakefield, Virginia forecast office.
    #[serde(rename = "AKQ")]
    Akq,
    /// The `ALY` identifier for the Albany, New York forecast office.
    #[serde(rename = "ALY")]
    Aly,
    /// The `BGM` identifier for the Binghamton, New York forecast office.
    #[serde(rename = "BGM")]
    Bgm,
    /// The `BOX` identifier for the Boston/Norton, Massachusetts forecast office.
    #[serde(rename = "BOX")]
    Box,
    /// The `BTV` identifier for the Burlington, Vermont forecast office.
    #[serde(rename = "BTV")]
    Btv,
    /// The `BUF` identifier for the Buffalo, New York forecast office.
    #[serde(rename = "BUF")]
    Buf,
    /// The `CAE` identifier for the Columbia, South Carolina forecast office.
    #[serde(rename = "CAE")]
    Cae,
    /// The `CAR` identifier for the Caribou, Maine forecast office.
    #[serde(rename = "CAR")]
    Car,
    /// The `CHS` identifier for the Charleston, South Carolina forecast office.
    #[serde(rename = "CHS")]
    Chs,
    /// The `CLE` identifier for the Cleveland, Ohio forecast office.
    #[serde(rename = "CLE")]
    Cle,
    /// The `CTP` identifier for the State College, Pennsylvania forecast office.
    #[serde(rename = "CTP")]
    Ctp,
    /// The `GSP` identifier for the Greenville-Spartanburg, South Carolina forecast office.
    #[serde(rename = "GSP")]
    Gsp,
    /// The `GYX` identifier for the Gray/Portland, Maine forecast office.
    #[serde(rename = "GYX")]
    Gyx,
    /// The `ILM` identifier for the Wilmington, North Carolina forecast office.
    #[serde(rename = "ILM")]
    Ilm,
    /// The `ILN` identifier for the Wilmington, Ohio forecast office.
    #[serde(rename = "ILN")]
    Iln,
    /// The `LWX` identifier for the Baltimore/Washington forecast office.
    #[serde(rename = "LWX")]
    Lwx,
    /// The `MHX` identifier for the Newport/Morehead City, North Carolina forecast office.
    #[serde(rename = "MHX")]
    Mhx,
    /// The `OKX` identifier for the New York, New York forecast office.
    #[serde(rename = "OKX")]
    Okx,
    /// The `PBZ` identifier for the Pittsburgh, Pennsylvania forecast office.
    #[serde(rename = "PBZ")]
    Pbz,
    /// The `PHI` identifier for the Philadelphia/Mount Holly forecast office.
    #[serde(rename = "PHI")]
    Phi,
    /// The `RAH` identifier for the Raleigh, North Carolina forecast office.
    #[serde(rename = "RAH")]
    Rah,
    /// The `RLX` identifier for the Charleston, West Virginia forecast office.
    #[serde(rename = "RLX")]
    Rlx,
    /// The `RNK` identifier for the Blacksburg, Virginia forecast office.
    #[serde(rename = "RNK")]
    Rnk,
    /// The `ABQ` identifier for the Albuquerque, New Mexico forecast office.
    #[serde(rename = "ABQ")]
    Abq,
    /// The `AMA` identifier for the Amarillo, Texas forecast office.
    #[serde(rename = "AMA")]
    Ama,
    /// The `BMX` identifier for the Birmingham, Alabama forecast office.
    #[serde(rename = "BMX")]
    Bmx,
    /// The `BRO` identifier for the Brownsville/Rio Grande Valley, Texas forecast office.
    #[serde(rename = "BRO")]
    Bro,
    /// The `CRP` identifier for the Corpus Christi, Texas forecast office.
    #[serde(rename = "CRP")]
    Crp,
    /// The `EPZ` identifier for the El Paso, Texas forecast office.
    #[serde(rename = "EPZ")]
    Epz,
    /// The `EWX` identifier for the Austin/San Antonio, Texas forecast office.
    #[serde(rename = "EWX")]
    Ewx,
    /// The `FFC` identifier for the Atlanta/Peachtree City, Georgia forecast office.
    #[serde(rename = "FFC")]
    Ffc,
    /// The `FWD` identifier for the Fort Worth/Dallas, Texas forecast office.
    #[serde(rename = "FWD")]
    Fwd,
    /// The `HGX` identifier for the Houston/Galveston, Texas forecast office.
    #[serde(rename = "HGX")]
    Hgx,
    /// The `HUN` identifier for the Huntsville, Alabama forecast office.
    #[serde(rename = "HUN")]
    Hun,
    /// The `JAN` identifier for the Jackson, Mississippi forecast office.
    #[serde(rename = "JAN")]
    Jan,
    /// The `JAX` identifier for the Jacksonville, Florida forecast office.
    #[serde(rename = "JAX")]
    Jax,
    /// The `KEY` identifier for the Key West, Florida forecast office.
    #[serde(rename = "KEY")]
    Key,
    /// The `LCH` identifier for the Lake Charles, Louisiana forecast office.
    #[serde(rename = "LCH")]
    Lch,
    /// The `LIX` identifier for the New Orleans/Baton Rouge forecast office.
    #[serde(rename = "LIX")]
    Lix,
    /// The `LUB` identifier for the Lubbock, Texas forecast office.
    #[serde(rename = "LUB")]
    Lub,
    /// The `LZK` identifier for the Little Rock, Arkansas forecast office.
    #[serde(rename = "LZK")]
    Lzk,
    /// The `MAF` identifier for the Midland/Odessa forecast office.
    #[serde(rename = "MAF")]
    Maf,
    /// The `MEG` identifier for the Memphis, Tennessee forecast office.
    #[serde(rename = "MEG")]
    Meg,
    /// The `MFL` identifier for the Miami/South Florida forecast office.
    #[serde(rename = "MFL")]
    Mfl,
    /// The `MLB` identifier for the Melbourne, Florida forecast office.
    #[serde(rename = "MLB")]
    Mlb,
    /// The `MOB` identifier for the Mobile/Pensacola forecast office.
    #[serde(rename = "MOB")]
    Mob,
    /// The `MRX` identifier for the Morristown, Tennessee forecast office.
    #[serde(rename = "MRX")]
    Mrx,
    /// The `OHX` identifier for the Nashville, Tennessee forecast office.
    #[serde(rename = "OHX")]
    Ohx,
    /// The `OUN` identifier for the Norman, Oklahoma forecast office.
    #[serde(rename = "OUN")]
    Oun,
    /// The `SHV` identifier for the Shreveport, Louisiana forecast office.
    #[serde(rename = "SHV")]
    Shv,
    /// The `SJT` identifier for the San Angelo, Texas forecast office.
    #[serde(rename = "SJT")]
    Sjt,
    /// The `SJU` identifier for the San Juan, Puerto Rico forecast office.
    #[serde(rename = "SJU")]
    Sju,
    /// The `TAE` identifier for the Tallahassee, Florida forecast office.
    #[serde(rename = "TAE")]
    Tae,
    /// The `TBW` identifier for the Tampa Bay Area, Florida forecast office.
    #[serde(rename = "TBW")]
    Tbw,
    /// The `TSA` identifier for the Tulsa, Oklahoma forecast office.
    #[serde(rename = "TSA")]
    Tsa,
    /// The `ABR` identifier for the Aberdeen, South Dakota forecast office.
    #[serde(rename = "ABR")]
    Abr,
    /// The `APX` identifier for the Gaylord, Michigan forecast office.
    #[serde(rename = "APX")]
    Apx,
    /// The `ARX` identifier for the La Crosse, Wisconsin forecast office.
    #[serde(rename = "ARX")]
    Arx,
    /// The `BIS` identifier for the Bismarck, North Dakota forecast office.
    #[serde(rename = "BIS")]
    Bis,
    /// The `BOU` identifier for the Denver/Boulder, Colorado forecast office.
    #[serde(rename = "BOU")]
    Bou,
    /// The `CYS` identifier for the Cheyenne, Wyoming forecast office.
    #[serde(rename = "CYS")]
    Cys,
    /// The `DDC` identifier for the Dodge City, Kansas forecast office.
    #[serde(rename = "DDC")]
    Ddc,
    /// The `DLH` identifier for the Duluth, Minnesota forecast office.
    #[serde(rename = "DLH")]
    Dlh,
    /// The `DMX` identifier for the Des Moines, Iowa forecast office.
    #[serde(rename = "DMX")]
    Dmx,
    /// The `DTX` identifier for the Detroit/Pontiac, Michigan forecast office.
    #[serde(rename = "DTX")]
    Dtx,
    /// The `DVN` identifier for the Quad Cities, Iowa/Illinois forecast office.
    #[serde(rename = "DVN")]
    Dvn,
    /// The `EAX` identifier for the Kansas City/Pleasant Hill, Missouri forecast office.
    #[serde(rename = "EAX")]
    Eax,
    /// The `FGF` identifier for the Grand Forks, North Dakota forecast office.
    #[serde(rename = "FGF")]
    Fgf,
    /// The `FSD` identifier for the Sioux Falls, South Dakota forecast office.
    #[serde(rename = "FSD")]
    Fsd,
    /// The `GID` identifier for the Hastings, Nebraska forecast office.
    #[serde(rename = "GID")]
    Gid,
    /// The `GJT` identifier for the Grand Junction, Colorado forecast office.
    #[serde(rename = "GJT")]
    Gjt,
    /// The `GLD` identifier for the Goodland, Kansas forecast office.
    #[serde(rename = "GLD")]
    Gld,
    /// The `GRB` identifier for the Green Bay, Wisconsin forecast office.
    #[serde(rename = "GRB")]
    Grb,
    /// The `GRR` identifier for the Grand Rapids, Michigan forecast office.
    #[serde(rename = "GRR")]
    Grr,
    /// The `ICT` identifier for the Wichita, Kansas forecast office.
    #[serde(rename = "ICT")]
    Ict,
    /// The `ILX` identifier for the Central Illinois forecast office.
    #[serde(rename = "ILX")]
    Ilx,
    /// The `IND` identifier for the Indianapolis, Indiana forecast office.
    #[serde(rename = "IND")]
    Ind,
    /// The `IWX` identifier for the Northern Indiana forecast office.
    #[serde(rename = "IWX")]
    Iwx,
    /// The `JKL` identifier for the Jackson, Kentucky forecast office.
    #[serde(rename = "JKL")]
    Jkl,
    /// The `LBF` identifier for the North Platte, Nebraska forecast office.
    #[serde(rename = "LBF")]
    Lbf,
    /// The `LMK` identifier for the Louisville, Kentucky forecast office.
    #[serde(rename = "LMK")]
    Lmk,
    /// The `LOT` identifier for the Chicago, Illinois forecast office.
    #[serde(rename = "LOT")]
    Lot,
    /// The `LSX` identifier for the St. Louis, Missouri forecast office.
    #[serde(rename = "LSX")]
    Lsx,
    /// The `MKX` identifier for the Milwaukee/Sullivan, Wisconsin forecast office.
    #[serde(rename = "MKX")]
    Mkx,
    /// The `MPX` identifier for the Twin Cities, Minnesota forecast office.
    #[serde(rename = "MPX")]
    Mpx,
    /// The `MQT` identifier for the Marquette, Michigan forecast office.
    #[serde(rename = "MQT")]
    Mqt,
    /// The `OAX` identifier for the Omaha/Valley, Nebraska forecast office.
    #[serde(rename = "OAX")]
    Oax,
    /// The `PAH` identifier for the Paducah, Kentucky forecast office.
    #[serde(rename = "PAH")]
    Pah,
    /// The `PUB` identifier for the Pueblo, Colorado forecast office.
    #[serde(rename = "PUB")]
    Pub,
    /// The `RIW` identifier for the Western and Central Wyoming forecast office.
    #[serde(rename = "RIW")]
    Riw,
    /// The `SGF` identifier for the Springfield, Missouri forecast office.
    #[serde(rename = "SGF")]
    Sgf,
    /// The `TOP` identifier for the Topeka, Kansas forecast office.
    #[serde(rename = "TOP")]
    Top,
    /// The `UNR` identifier for the Rapid City, South Dakota forecast office.
    #[serde(rename = "UNR")]
    Unr,
    /// The `BOI` identifier for the Boise, Idaho forecast office.
    #[serde(rename = "BOI")]
    Boi,
    /// The `BYZ` identifier for the Billings, Montana forecast office.
    #[serde(rename = "BYZ")]
    Byz,
    /// The `EKA` identifier for the Eureka, California forecast office.
    #[serde(rename = "EKA")]
    Eka,
    /// The `FGZ` identifier for the Flagstaff, Arizona forecast office.
    #[serde(rename = "FGZ")]
    Fgz,
    /// The `GGW` identifier for the Glasgow, Montana forecast office.
    #[serde(rename = "GGW")]
    Ggw,
    /// The `HNX` identifier for the San Joaquin Valley, California forecast office.
    #[serde(rename = "HNX")]
    Hnx,
    /// The `LKN` identifier for the Elko, Nevada forecast office.
    #[serde(rename = "LKN")]
    Lkn,
    /// The `LOX` identifier for the Los Angeles, California forecast office.
    #[serde(rename = "LOX")]
    Lox,
    /// The `MFR` identifier for the Medford, Oregon forecast office.
    #[serde(rename = "MFR")]
    Mfr,
    /// The `MSO` identifier for the Missoula, Montana forecast office.
    #[serde(rename = "MSO")]
    Mso,
    /// The `MTR` identifier for the San Francisco Bay Area forecast office.
    #[serde(rename = "MTR")]
    Mtr,
    /// The `OTX` identifier for the Spokane, Washington forecast office.
    #[serde(rename = "OTX")]
    Otx,
    /// The `PDT` identifier for the Pendleton, Oregon forecast office.
    #[serde(rename = "PDT")]
    Pdt,
    /// The `PIH` identifier for the Pocatello, Idaho forecast office.
    #[serde(rename = "PIH")]
    Pih,
    /// The `PQR` identifier for the Portland, Oregon forecast office.
    #[serde(rename = "PQR")]
    Pqr,
    /// The `PSR` identifier for the Phoenix, Arizona forecast office.
    #[serde(rename = "PSR")]
    Psr,
    /// The `REV` identifier for the Reno, Nevada forecast office.
    #[serde(rename = "REV")]
    Rev,
    /// The `SEW` identifier for the Seattle/Tacoma, Washington forecast office.
    #[serde(rename = "SEW")]
    Sew,
    /// The `SGX` identifier for the San Diego, California forecast office.
    #[serde(rename = "SGX")]
    Sgx,
    /// The `SLC` identifier for the Salt Lake City, Utah forecast office.
    #[serde(rename = "SLC")]
    Slc,
    /// The `STO` identifier for the Sacramento, California forecast office.
    #[serde(rename = "STO")]
    Sto,
    /// The `TFX` identifier for the Great Falls, Montana forecast office.
    #[serde(rename = "TFX")]
    Tfx,
    /// The `TWC` identifier for the Tucson, Arizona forecast office.
    #[serde(rename = "TWC")]
    Twc,
    /// The `VEF` identifier for the Las Vegas, Nevada forecast office.
    #[serde(rename = "VEF")]
    Vef,
    /// The `AER` identifier for the Anchorage East forecast domain.
    #[serde(rename = "AER")]
    Aer,
    /// The `AFC` identifier for the Anchorage, Alaska forecast office.
    #[serde(rename = "AFC")]
    Afc,
    /// The `AFG` identifier for the Fairbanks, Alaska forecast office.
    #[serde(rename = "AFG")]
    Afg,
    /// The `AJK` identifier for the Juneau, Alaska forecast office.
    #[serde(rename = "AJK")]
    Ajk,
    /// The `ALU` identifier for the Anchorage West forecast domain.
    #[serde(rename = "ALU")]
    Alu,
    /// The `GUM` identifier for the Tiyan, Guam forecast office.
    #[serde(rename = "GUM")]
    Gum,
    /// The `HPA` identifier for the Hawaiian offshore waters forecast domain.
    #[serde(rename = "HPA")]
    Hpa,
    /// The `HFO` identifier for the Honolulu, Hawaii forecast office.
    #[serde(rename = "HFO")]
    Hfo,
    /// The `PPG` identifier for the Pago Pago, American Samoa weather office.
    #[serde(rename = "PPG")]
    Ppg,
    /// The `STU` identifier for the Pago Pago, American Samoa weather office.
    #[serde(rename = "STU")]
    Stu,
    /// The `NH1` identifier for the National Hurricane Center eastern Pacific domain.
    #[serde(rename = "NH1")]
    Nh1,
    /// The `NH2` identifier for the National Hurricane Center Atlantic domain.
    #[serde(rename = "NH2")]
    Nh2,
    /// The `ONA` identifier for the Ocean Prediction Center Atlantic domain.
    #[serde(rename = "ONA")]
    Ona,
    /// The `ONP` identifier for the Ocean Prediction Center Pacific domain.
    #[serde(rename = "ONP")]
    Onp,
    /// The `PQE` identifier for eastern Micronesia and the Marshall Islands.
    #[serde(rename = "PQE")]
    Pqe,
    /// The `PQW` identifier for western Micronesia and Palau.
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
    fn test_from_str_pqe() {
        assert_eq!(
            "PQE".parse::<NwsForecastOfficeId>().unwrap(),
            NwsForecastOfficeId::Pqe
        );
    }

    #[test]
    fn test_from_str_pqw() {
        assert_eq!(
            "PQW".parse::<NwsForecastOfficeId>().unwrap(),
            NwsForecastOfficeId::Pqw
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
