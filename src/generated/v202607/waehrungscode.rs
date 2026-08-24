#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Aufzählung der Währungscodes.
#[non_exhaustive]
pub enum Waehrungscode {
    #[cfg_attr(feature = "serde", serde(rename = "AED"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AED"))]
    Aed,
    #[cfg_attr(feature = "serde", serde(rename = "AFN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AFN"))]
    Afn,
    #[cfg_attr(feature = "serde", serde(rename = "ALL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ALL"))]
    All,
    #[cfg_attr(feature = "serde", serde(rename = "AMD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AMD"))]
    Amd,
    #[cfg_attr(feature = "serde", serde(rename = "ANG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ANG"))]
    Ang,
    #[cfg_attr(feature = "serde", serde(rename = "AOA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AOA"))]
    Aoa,
    #[cfg_attr(feature = "serde", serde(rename = "ARS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ARS"))]
    Ars,
    #[cfg_attr(feature = "serde", serde(rename = "AUD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AUD"))]
    Aud,
    #[cfg_attr(feature = "serde", serde(rename = "AWG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AWG"))]
    Awg,
    #[cfg_attr(feature = "serde", serde(rename = "AZN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AZN"))]
    Azn,
    #[cfg_attr(feature = "serde", serde(rename = "BAM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BAM"))]
    Bam,
    #[cfg_attr(feature = "serde", serde(rename = "BBD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BBD"))]
    Bbd,
    #[cfg_attr(feature = "serde", serde(rename = "BDT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BDT"))]
    Bdt,
    #[cfg_attr(feature = "serde", serde(rename = "BGN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BGN"))]
    Bgn,
    #[cfg_attr(feature = "serde", serde(rename = "BHD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BHD"))]
    Bhd,
    #[cfg_attr(feature = "serde", serde(rename = "BIF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BIF"))]
    Bif,
    #[cfg_attr(feature = "serde", serde(rename = "BMD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BMD"))]
    Bmd,
    #[cfg_attr(feature = "serde", serde(rename = "BND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BND"))]
    Bnd,
    #[cfg_attr(feature = "serde", serde(rename = "BOB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BOB"))]
    Bob,
    #[cfg_attr(feature = "serde", serde(rename = "BOV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BOV"))]
    Bov,
    #[cfg_attr(feature = "serde", serde(rename = "BRL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BRL"))]
    Brl,
    #[cfg_attr(feature = "serde", serde(rename = "BSD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BSD"))]
    Bsd,
    #[cfg_attr(feature = "serde", serde(rename = "BTN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BTN"))]
    Btn,
    #[cfg_attr(feature = "serde", serde(rename = "BWP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BWP"))]
    Bwp,
    #[cfg_attr(feature = "serde", serde(rename = "BYN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BYN"))]
    Byn,
    #[cfg_attr(feature = "serde", serde(rename = "BYR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BYR"))]
    Byr,
    #[cfg_attr(feature = "serde", serde(rename = "BZD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BZD"))]
    Bzd,
    #[cfg_attr(feature = "serde", serde(rename = "CAD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CAD"))]
    Cad,
    #[cfg_attr(feature = "serde", serde(rename = "CDF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CDF"))]
    Cdf,
    #[cfg_attr(feature = "serde", serde(rename = "CHE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CHE"))]
    Che,
    #[cfg_attr(feature = "serde", serde(rename = "CHF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CHF"))]
    Chf,
    #[cfg_attr(feature = "serde", serde(rename = "CHW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CHW"))]
    Chw,
    #[cfg_attr(feature = "serde", serde(rename = "CLF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CLF"))]
    Clf,
    #[cfg_attr(feature = "serde", serde(rename = "CLP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CLP"))]
    Clp,
    #[cfg_attr(feature = "serde", serde(rename = "CNY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CNY"))]
    Cny,
    #[cfg_attr(feature = "serde", serde(rename = "COP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "COP"))]
    Cop,
    #[cfg_attr(feature = "serde", serde(rename = "COU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "COU"))]
    Cou,
    #[cfg_attr(feature = "serde", serde(rename = "CRC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CRC"))]
    Crc,
    #[cfg_attr(feature = "serde", serde(rename = "CUC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CUC"))]
    Cuc,
    #[cfg_attr(feature = "serde", serde(rename = "CUP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CUP"))]
    Cup,
    #[cfg_attr(feature = "serde", serde(rename = "CVE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CVE"))]
    Cve,
    #[cfg_attr(feature = "serde", serde(rename = "CZK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CZK"))]
    Czk,
    #[cfg_attr(feature = "serde", serde(rename = "DJF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DJF"))]
    Djf,
    #[cfg_attr(feature = "serde", serde(rename = "DKK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DKK"))]
    Dkk,
    #[cfg_attr(feature = "serde", serde(rename = "DOP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DOP"))]
    Dop,
    #[cfg_attr(feature = "serde", serde(rename = "DZD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DZD"))]
    Dzd,
    #[cfg_attr(feature = "serde", serde(rename = "EGP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EGP"))]
    Egp,
    #[cfg_attr(feature = "serde", serde(rename = "ERN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ERN"))]
    Ern,
    #[cfg_attr(feature = "serde", serde(rename = "ETB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ETB"))]
    Etb,
    #[cfg_attr(feature = "serde", serde(rename = "EUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EUR"))]
    Eur,
    #[cfg_attr(feature = "serde", serde(rename = "FJD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FJD"))]
    Fjd,
    #[cfg_attr(feature = "serde", serde(rename = "FKP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FKP"))]
    Fkp,
    #[cfg_attr(feature = "serde", serde(rename = "GBP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GBP"))]
    Gbp,
    #[cfg_attr(feature = "serde", serde(rename = "GEL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GEL"))]
    Gel,
    #[cfg_attr(feature = "serde", serde(rename = "GHS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GHS"))]
    Ghs,
    #[cfg_attr(feature = "serde", serde(rename = "GIP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GIP"))]
    Gip,
    #[cfg_attr(feature = "serde", serde(rename = "GMD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GMD"))]
    Gmd,
    #[cfg_attr(feature = "serde", serde(rename = "GNF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GNF"))]
    Gnf,
    #[cfg_attr(feature = "serde", serde(rename = "GTQ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GTQ"))]
    Gtq,
    #[cfg_attr(feature = "serde", serde(rename = "GYD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GYD"))]
    Gyd,
    #[cfg_attr(feature = "serde", serde(rename = "HKD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HKD"))]
    Hkd,
    #[cfg_attr(feature = "serde", serde(rename = "HNL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HNL"))]
    Hnl,
    #[cfg_attr(feature = "serde", serde(rename = "HRK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HRK"))]
    Hrk,
    #[cfg_attr(feature = "serde", serde(rename = "HTG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HTG"))]
    Htg,
    #[cfg_attr(feature = "serde", serde(rename = "HUF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HUF"))]
    Huf,
    #[cfg_attr(feature = "serde", serde(rename = "IDR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IDR"))]
    Idr,
    #[cfg_attr(feature = "serde", serde(rename = "ILS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ILS"))]
    Ils,
    #[cfg_attr(feature = "serde", serde(rename = "INR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "INR"))]
    Inr,
    #[cfg_attr(feature = "serde", serde(rename = "IQD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IQD"))]
    Iqd,
    #[cfg_attr(feature = "serde", serde(rename = "IRR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IRR"))]
    Irr,
    #[cfg_attr(feature = "serde", serde(rename = "ISK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ISK"))]
    Isk,
    #[cfg_attr(feature = "serde", serde(rename = "JMD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JMD"))]
    Jmd,
    #[cfg_attr(feature = "serde", serde(rename = "JOD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JOD"))]
    Jod,
    #[cfg_attr(feature = "serde", serde(rename = "JPY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JPY"))]
    Jpy,
    #[cfg_attr(feature = "serde", serde(rename = "KES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KES"))]
    Kes,
    #[cfg_attr(feature = "serde", serde(rename = "KGS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KGS"))]
    Kgs,
    #[cfg_attr(feature = "serde", serde(rename = "KHR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KHR"))]
    Khr,
    #[cfg_attr(feature = "serde", serde(rename = "KMF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KMF"))]
    Kmf,
    #[cfg_attr(feature = "serde", serde(rename = "KPW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KPW"))]
    Kpw,
    #[cfg_attr(feature = "serde", serde(rename = "KRW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KRW"))]
    Krw,
    #[cfg_attr(feature = "serde", serde(rename = "KWD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KWD"))]
    Kwd,
    #[cfg_attr(feature = "serde", serde(rename = "KYD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KYD"))]
    Kyd,
    #[cfg_attr(feature = "serde", serde(rename = "KZT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KZT"))]
    Kzt,
    #[cfg_attr(feature = "serde", serde(rename = "LAK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LAK"))]
    Lak,
    #[cfg_attr(feature = "serde", serde(rename = "LBP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LBP"))]
    Lbp,
    #[cfg_attr(feature = "serde", serde(rename = "LKR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LKR"))]
    Lkr,
    #[cfg_attr(feature = "serde", serde(rename = "LRD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LRD"))]
    Lrd,
    #[cfg_attr(feature = "serde", serde(rename = "LSL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LSL"))]
    Lsl,
    #[cfg_attr(feature = "serde", serde(rename = "LTL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LTL"))]
    Ltl,
    #[cfg_attr(feature = "serde", serde(rename = "LYD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LYD"))]
    Lyd,
    #[cfg_attr(feature = "serde", serde(rename = "MAD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MAD"))]
    Mad,
    #[cfg_attr(feature = "serde", serde(rename = "MDL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MDL"))]
    Mdl,
    #[cfg_attr(feature = "serde", serde(rename = "MGA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MGA"))]
    Mga,
    #[cfg_attr(feature = "serde", serde(rename = "MKD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MKD"))]
    Mkd,
    #[cfg_attr(feature = "serde", serde(rename = "MMK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MMK"))]
    Mmk,
    #[cfg_attr(feature = "serde", serde(rename = "MNT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MNT"))]
    Mnt,
    #[cfg_attr(feature = "serde", serde(rename = "MOP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MOP"))]
    Mop,
    #[cfg_attr(feature = "serde", serde(rename = "MRO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MRO"))]
    Mro,
    #[cfg_attr(feature = "serde", serde(rename = "MUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MUR"))]
    Mur,
    #[cfg_attr(feature = "serde", serde(rename = "MVR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MVR"))]
    Mvr,
    #[cfg_attr(feature = "serde", serde(rename = "MWK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MWK"))]
    Mwk,
    #[cfg_attr(feature = "serde", serde(rename = "MXN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MXN"))]
    Mxn,
    #[cfg_attr(feature = "serde", serde(rename = "MXV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MXV"))]
    Mxv,
    #[cfg_attr(feature = "serde", serde(rename = "MYR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MYR"))]
    Myr,
    #[cfg_attr(feature = "serde", serde(rename = "MZN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MZN"))]
    Mzn,
    #[cfg_attr(feature = "serde", serde(rename = "NAD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NAD"))]
    Nad,
    #[cfg_attr(feature = "serde", serde(rename = "NGN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NGN"))]
    Ngn,
    #[cfg_attr(feature = "serde", serde(rename = "NIO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NIO"))]
    Nio,
    #[cfg_attr(feature = "serde", serde(rename = "NOK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NOK"))]
    Nok,
    #[cfg_attr(feature = "serde", serde(rename = "NPR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NPR"))]
    Npr,
    #[cfg_attr(feature = "serde", serde(rename = "NZD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NZD"))]
    Nzd,
    #[cfg_attr(feature = "serde", serde(rename = "OMR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OMR"))]
    Omr,
    #[cfg_attr(feature = "serde", serde(rename = "PAB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PAB"))]
    Pab,
    #[cfg_attr(feature = "serde", serde(rename = "PEN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PEN"))]
    Pen,
    #[cfg_attr(feature = "serde", serde(rename = "PGK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PGK"))]
    Pgk,
    #[cfg_attr(feature = "serde", serde(rename = "PHP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PHP"))]
    Php,
    #[cfg_attr(feature = "serde", serde(rename = "PKR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PKR"))]
    Pkr,
    #[cfg_attr(feature = "serde", serde(rename = "PLN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PLN"))]
    Pln,
    #[cfg_attr(feature = "serde", serde(rename = "PYG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PYG"))]
    Pyg,
    #[cfg_attr(feature = "serde", serde(rename = "QAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "QAR"))]
    Qar,
    #[cfg_attr(feature = "serde", serde(rename = "RON"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RON"))]
    Ron,
    #[cfg_attr(feature = "serde", serde(rename = "RSD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RSD"))]
    Rsd,
    #[cfg_attr(feature = "serde", serde(rename = "RUB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RUB"))]
    Rub,
    #[cfg_attr(feature = "serde", serde(rename = "RUR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RUR"))]
    Rur,
    #[cfg_attr(feature = "serde", serde(rename = "RWF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RWF"))]
    Rwf,
    #[cfg_attr(feature = "serde", serde(rename = "SAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SAR"))]
    Sar,
    #[cfg_attr(feature = "serde", serde(rename = "SBD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SBD"))]
    Sbd,
    #[cfg_attr(feature = "serde", serde(rename = "SCR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SCR"))]
    Scr,
    #[cfg_attr(feature = "serde", serde(rename = "SDG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SDG"))]
    Sdg,
    #[cfg_attr(feature = "serde", serde(rename = "SEK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SEK"))]
    Sek,
    #[cfg_attr(feature = "serde", serde(rename = "SGD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SGD"))]
    Sgd,
    #[cfg_attr(feature = "serde", serde(rename = "SHP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SHP"))]
    Shp,
    #[cfg_attr(feature = "serde", serde(rename = "SLL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SLL"))]
    Sll,
    #[cfg_attr(feature = "serde", serde(rename = "SOS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SOS"))]
    Sos,
    #[cfg_attr(feature = "serde", serde(rename = "SRD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SRD"))]
    Srd,
    #[cfg_attr(feature = "serde", serde(rename = "SSP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SSP"))]
    Ssp,
    #[cfg_attr(feature = "serde", serde(rename = "STD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "STD"))]
    Std,
    #[cfg_attr(feature = "serde", serde(rename = "SVC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SVC"))]
    Svc,
    #[cfg_attr(feature = "serde", serde(rename = "SYP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SYP"))]
    Syp,
    #[cfg_attr(feature = "serde", serde(rename = "SZL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SZL"))]
    Szl,
    #[cfg_attr(feature = "serde", serde(rename = "THB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "THB"))]
    Thb,
    #[cfg_attr(feature = "serde", serde(rename = "TJS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TJS"))]
    Tjs,
    #[cfg_attr(feature = "serde", serde(rename = "TMT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TMT"))]
    Tmt,
    #[cfg_attr(feature = "serde", serde(rename = "TND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TND"))]
    Tnd,
    #[cfg_attr(feature = "serde", serde(rename = "TOP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TOP"))]
    Top,
    #[cfg_attr(feature = "serde", serde(rename = "TRY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TRY"))]
    Try,
    #[cfg_attr(feature = "serde", serde(rename = "TTD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TTD"))]
    Ttd,
    #[cfg_attr(feature = "serde", serde(rename = "TWD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TWD"))]
    Twd,
    #[cfg_attr(feature = "serde", serde(rename = "TZS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TZS"))]
    Tzs,
    #[cfg_attr(feature = "serde", serde(rename = "UAH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UAH"))]
    Uah,
    #[cfg_attr(feature = "serde", serde(rename = "UGX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UGX"))]
    Ugx,
    #[cfg_attr(feature = "serde", serde(rename = "USD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "USD"))]
    Usd,
    #[cfg_attr(feature = "serde", serde(rename = "USN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "USN"))]
    Usn,
    #[cfg_attr(feature = "serde", serde(rename = "USS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "USS"))]
    Uss,
    #[cfg_attr(feature = "serde", serde(rename = "UYI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UYI"))]
    Uyi,
    #[cfg_attr(feature = "serde", serde(rename = "UYU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UYU"))]
    Uyu,
    #[cfg_attr(feature = "serde", serde(rename = "UZS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UZS"))]
    Uzs,
    #[cfg_attr(feature = "serde", serde(rename = "VEF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VEF"))]
    Vef,
    #[cfg_attr(feature = "serde", serde(rename = "VND"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VND"))]
    Vnd,
    #[cfg_attr(feature = "serde", serde(rename = "VUV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VUV"))]
    Vuv,
    #[cfg_attr(feature = "serde", serde(rename = "WST"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WST"))]
    Wst,
    #[cfg_attr(feature = "serde", serde(rename = "XAF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XAF"))]
    Xaf,
    #[cfg_attr(feature = "serde", serde(rename = "XAG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XAG"))]
    Xag,
    #[cfg_attr(feature = "serde", serde(rename = "XAU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XAU"))]
    Xau,
    #[cfg_attr(feature = "serde", serde(rename = "XBA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XBA"))]
    Xba,
    #[cfg_attr(feature = "serde", serde(rename = "XBB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XBB"))]
    Xbb,
    #[cfg_attr(feature = "serde", serde(rename = "XBC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XBC"))]
    Xbc,
    #[cfg_attr(feature = "serde", serde(rename = "XBD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XBD"))]
    Xbd,
    #[cfg_attr(feature = "serde", serde(rename = "XCD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XCD"))]
    Xcd,
    #[cfg_attr(feature = "serde", serde(rename = "XDR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XDR"))]
    Xdr,
    #[cfg_attr(feature = "serde", serde(rename = "XOF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XOF"))]
    Xof,
    #[cfg_attr(feature = "serde", serde(rename = "XPD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XPD"))]
    Xpd,
    #[cfg_attr(feature = "serde", serde(rename = "XPF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XPF"))]
    Xpf,
    #[cfg_attr(feature = "serde", serde(rename = "XPT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XPT"))]
    Xpt,
    #[cfg_attr(feature = "serde", serde(rename = "XSU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XSU"))]
    Xsu,
    #[cfg_attr(feature = "serde", serde(rename = "XTS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XTS"))]
    Xts,
    #[cfg_attr(feature = "serde", serde(rename = "XUA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XUA"))]
    Xua,
    #[cfg_attr(feature = "serde", serde(rename = "XXX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XXX"))]
    Xxx,
    #[cfg_attr(feature = "serde", serde(rename = "YER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "YER"))]
    Yer,
    #[cfg_attr(feature = "serde", serde(rename = "ZAR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZAR"))]
    Zar,
    #[cfg_attr(feature = "serde", serde(rename = "ZMW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZMW"))]
    Zmw,
    #[cfg_attr(feature = "serde", serde(rename = "ZWL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZWL"))]
    Zwl,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Waehrungscode {
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Waehrungscode::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Aed,
        Self::Afn,
        Self::All,
        Self::Amd,
        Self::Ang,
        Self::Aoa,
        Self::Ars,
        Self::Aud,
        Self::Awg,
        Self::Azn,
        Self::Bam,
        Self::Bbd,
        Self::Bdt,
        Self::Bgn,
        Self::Bhd,
        Self::Bif,
        Self::Bmd,
        Self::Bnd,
        Self::Bob,
        Self::Bov,
        Self::Brl,
        Self::Bsd,
        Self::Btn,
        Self::Bwp,
        Self::Byn,
        Self::Byr,
        Self::Bzd,
        Self::Cad,
        Self::Cdf,
        Self::Che,
        Self::Chf,
        Self::Chw,
        Self::Clf,
        Self::Clp,
        Self::Cny,
        Self::Cop,
        Self::Cou,
        Self::Crc,
        Self::Cuc,
        Self::Cup,
        Self::Cve,
        Self::Czk,
        Self::Djf,
        Self::Dkk,
        Self::Dop,
        Self::Dzd,
        Self::Egp,
        Self::Ern,
        Self::Etb,
        Self::Eur,
        Self::Fjd,
        Self::Fkp,
        Self::Gbp,
        Self::Gel,
        Self::Ghs,
        Self::Gip,
        Self::Gmd,
        Self::Gnf,
        Self::Gtq,
        Self::Gyd,
        Self::Hkd,
        Self::Hnl,
        Self::Hrk,
        Self::Htg,
        Self::Huf,
        Self::Idr,
        Self::Ils,
        Self::Inr,
        Self::Iqd,
        Self::Irr,
        Self::Isk,
        Self::Jmd,
        Self::Jod,
        Self::Jpy,
        Self::Kes,
        Self::Kgs,
        Self::Khr,
        Self::Kmf,
        Self::Kpw,
        Self::Krw,
        Self::Kwd,
        Self::Kyd,
        Self::Kzt,
        Self::Lak,
        Self::Lbp,
        Self::Lkr,
        Self::Lrd,
        Self::Lsl,
        Self::Ltl,
        Self::Lyd,
        Self::Mad,
        Self::Mdl,
        Self::Mga,
        Self::Mkd,
        Self::Mmk,
        Self::Mnt,
        Self::Mop,
        Self::Mro,
        Self::Mur,
        Self::Mvr,
        Self::Mwk,
        Self::Mxn,
        Self::Mxv,
        Self::Myr,
        Self::Mzn,
        Self::Nad,
        Self::Ngn,
        Self::Nio,
        Self::Nok,
        Self::Npr,
        Self::Nzd,
        Self::Omr,
        Self::Pab,
        Self::Pen,
        Self::Pgk,
        Self::Php,
        Self::Pkr,
        Self::Pln,
        Self::Pyg,
        Self::Qar,
        Self::Ron,
        Self::Rsd,
        Self::Rub,
        Self::Rur,
        Self::Rwf,
        Self::Sar,
        Self::Sbd,
        Self::Scr,
        Self::Sdg,
        Self::Sek,
        Self::Sgd,
        Self::Shp,
        Self::Sll,
        Self::Sos,
        Self::Srd,
        Self::Ssp,
        Self::Std,
        Self::Svc,
        Self::Syp,
        Self::Szl,
        Self::Thb,
        Self::Tjs,
        Self::Tmt,
        Self::Tnd,
        Self::Top,
        Self::Try,
        Self::Ttd,
        Self::Twd,
        Self::Tzs,
        Self::Uah,
        Self::Ugx,
        Self::Usd,
        Self::Usn,
        Self::Uss,
        Self::Uyi,
        Self::Uyu,
        Self::Uzs,
        Self::Vef,
        Self::Vnd,
        Self::Vuv,
        Self::Wst,
        Self::Xaf,
        Self::Xag,
        Self::Xau,
        Self::Xba,
        Self::Xbb,
        Self::Xbc,
        Self::Xbd,
        Self::Xcd,
        Self::Xdr,
        Self::Xof,
        Self::Xpd,
        Self::Xpf,
        Self::Xpt,
        Self::Xsu,
        Self::Xts,
        Self::Xua,
        Self::Xxx,
        Self::Yer,
        Self::Zar,
        Self::Zmw,
        Self::Zwl,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Waehrungscode::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Waehrungscode`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Waehrungscode::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Waehrungscode;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Waehrungscode::iter_known().count(), Waehrungscode::COUNT);
    /// assert!(Waehrungscode::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Waehrungscode::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Aed => "AED",
            Self::Afn => "AFN",
            Self::All => "ALL",
            Self::Amd => "AMD",
            Self::Ang => "ANG",
            Self::Aoa => "AOA",
            Self::Ars => "ARS",
            Self::Aud => "AUD",
            Self::Awg => "AWG",
            Self::Azn => "AZN",
            Self::Bam => "BAM",
            Self::Bbd => "BBD",
            Self::Bdt => "BDT",
            Self::Bgn => "BGN",
            Self::Bhd => "BHD",
            Self::Bif => "BIF",
            Self::Bmd => "BMD",
            Self::Bnd => "BND",
            Self::Bob => "BOB",
            Self::Bov => "BOV",
            Self::Brl => "BRL",
            Self::Bsd => "BSD",
            Self::Btn => "BTN",
            Self::Bwp => "BWP",
            Self::Byn => "BYN",
            Self::Byr => "BYR",
            Self::Bzd => "BZD",
            Self::Cad => "CAD",
            Self::Cdf => "CDF",
            Self::Che => "CHE",
            Self::Chf => "CHF",
            Self::Chw => "CHW",
            Self::Clf => "CLF",
            Self::Clp => "CLP",
            Self::Cny => "CNY",
            Self::Cop => "COP",
            Self::Cou => "COU",
            Self::Crc => "CRC",
            Self::Cuc => "CUC",
            Self::Cup => "CUP",
            Self::Cve => "CVE",
            Self::Czk => "CZK",
            Self::Djf => "DJF",
            Self::Dkk => "DKK",
            Self::Dop => "DOP",
            Self::Dzd => "DZD",
            Self::Egp => "EGP",
            Self::Ern => "ERN",
            Self::Etb => "ETB",
            Self::Eur => "EUR",
            Self::Fjd => "FJD",
            Self::Fkp => "FKP",
            Self::Gbp => "GBP",
            Self::Gel => "GEL",
            Self::Ghs => "GHS",
            Self::Gip => "GIP",
            Self::Gmd => "GMD",
            Self::Gnf => "GNF",
            Self::Gtq => "GTQ",
            Self::Gyd => "GYD",
            Self::Hkd => "HKD",
            Self::Hnl => "HNL",
            Self::Hrk => "HRK",
            Self::Htg => "HTG",
            Self::Huf => "HUF",
            Self::Idr => "IDR",
            Self::Ils => "ILS",
            Self::Inr => "INR",
            Self::Iqd => "IQD",
            Self::Irr => "IRR",
            Self::Isk => "ISK",
            Self::Jmd => "JMD",
            Self::Jod => "JOD",
            Self::Jpy => "JPY",
            Self::Kes => "KES",
            Self::Kgs => "KGS",
            Self::Khr => "KHR",
            Self::Kmf => "KMF",
            Self::Kpw => "KPW",
            Self::Krw => "KRW",
            Self::Kwd => "KWD",
            Self::Kyd => "KYD",
            Self::Kzt => "KZT",
            Self::Lak => "LAK",
            Self::Lbp => "LBP",
            Self::Lkr => "LKR",
            Self::Lrd => "LRD",
            Self::Lsl => "LSL",
            Self::Ltl => "LTL",
            Self::Lyd => "LYD",
            Self::Mad => "MAD",
            Self::Mdl => "MDL",
            Self::Mga => "MGA",
            Self::Mkd => "MKD",
            Self::Mmk => "MMK",
            Self::Mnt => "MNT",
            Self::Mop => "MOP",
            Self::Mro => "MRO",
            Self::Mur => "MUR",
            Self::Mvr => "MVR",
            Self::Mwk => "MWK",
            Self::Mxn => "MXN",
            Self::Mxv => "MXV",
            Self::Myr => "MYR",
            Self::Mzn => "MZN",
            Self::Nad => "NAD",
            Self::Ngn => "NGN",
            Self::Nio => "NIO",
            Self::Nok => "NOK",
            Self::Npr => "NPR",
            Self::Nzd => "NZD",
            Self::Omr => "OMR",
            Self::Pab => "PAB",
            Self::Pen => "PEN",
            Self::Pgk => "PGK",
            Self::Php => "PHP",
            Self::Pkr => "PKR",
            Self::Pln => "PLN",
            Self::Pyg => "PYG",
            Self::Qar => "QAR",
            Self::Ron => "RON",
            Self::Rsd => "RSD",
            Self::Rub => "RUB",
            Self::Rur => "RUR",
            Self::Rwf => "RWF",
            Self::Sar => "SAR",
            Self::Sbd => "SBD",
            Self::Scr => "SCR",
            Self::Sdg => "SDG",
            Self::Sek => "SEK",
            Self::Sgd => "SGD",
            Self::Shp => "SHP",
            Self::Sll => "SLL",
            Self::Sos => "SOS",
            Self::Srd => "SRD",
            Self::Ssp => "SSP",
            Self::Std => "STD",
            Self::Svc => "SVC",
            Self::Syp => "SYP",
            Self::Szl => "SZL",
            Self::Thb => "THB",
            Self::Tjs => "TJS",
            Self::Tmt => "TMT",
            Self::Tnd => "TND",
            Self::Top => "TOP",
            Self::Try => "TRY",
            Self::Ttd => "TTD",
            Self::Twd => "TWD",
            Self::Tzs => "TZS",
            Self::Uah => "UAH",
            Self::Ugx => "UGX",
            Self::Usd => "USD",
            Self::Usn => "USN",
            Self::Uss => "USS",
            Self::Uyi => "UYI",
            Self::Uyu => "UYU",
            Self::Uzs => "UZS",
            Self::Vef => "VEF",
            Self::Vnd => "VND",
            Self::Vuv => "VUV",
            Self::Wst => "WST",
            Self::Xaf => "XAF",
            Self::Xag => "XAG",
            Self::Xau => "XAU",
            Self::Xba => "XBA",
            Self::Xbb => "XBB",
            Self::Xbc => "XBC",
            Self::Xbd => "XBD",
            Self::Xcd => "XCD",
            Self::Xdr => "XDR",
            Self::Xof => "XOF",
            Self::Xpd => "XPD",
            Self::Xpf => "XPF",
            Self::Xpt => "XPT",
            Self::Xsu => "XSU",
            Self::Xts => "XTS",
            Self::Xua => "XUA",
            Self::Xxx => "XXX",
            Self::Yer => "YER",
            Self::Zar => "ZAR",
            Self::Zmw => "ZMW",
            Self::Zwl => "ZWL",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Waehrungscode::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Waehrungscode;
    /// assert_eq!(Waehrungscode::from_wire("AED"), Ok(Waehrungscode::Aed));
    /// // Out-of-schema values are rejected rather than degraded:
    /// assert!(Waehrungscode::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Waehrungscode::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "AED" => Ok(Self::Aed),
            "AFN" => Ok(Self::Afn),
            "ALL" => Ok(Self::All),
            "AMD" => Ok(Self::Amd),
            "ANG" => Ok(Self::Ang),
            "AOA" => Ok(Self::Aoa),
            "ARS" => Ok(Self::Ars),
            "AUD" => Ok(Self::Aud),
            "AWG" => Ok(Self::Awg),
            "AZN" => Ok(Self::Azn),
            "BAM" => Ok(Self::Bam),
            "BBD" => Ok(Self::Bbd),
            "BDT" => Ok(Self::Bdt),
            "BGN" => Ok(Self::Bgn),
            "BHD" => Ok(Self::Bhd),
            "BIF" => Ok(Self::Bif),
            "BMD" => Ok(Self::Bmd),
            "BND" => Ok(Self::Bnd),
            "BOB" => Ok(Self::Bob),
            "BOV" => Ok(Self::Bov),
            "BRL" => Ok(Self::Brl),
            "BSD" => Ok(Self::Bsd),
            "BTN" => Ok(Self::Btn),
            "BWP" => Ok(Self::Bwp),
            "BYN" => Ok(Self::Byn),
            "BYR" => Ok(Self::Byr),
            "BZD" => Ok(Self::Bzd),
            "CAD" => Ok(Self::Cad),
            "CDF" => Ok(Self::Cdf),
            "CHE" => Ok(Self::Che),
            "CHF" => Ok(Self::Chf),
            "CHW" => Ok(Self::Chw),
            "CLF" => Ok(Self::Clf),
            "CLP" => Ok(Self::Clp),
            "CNY" => Ok(Self::Cny),
            "COP" => Ok(Self::Cop),
            "COU" => Ok(Self::Cou),
            "CRC" => Ok(Self::Crc),
            "CUC" => Ok(Self::Cuc),
            "CUP" => Ok(Self::Cup),
            "CVE" => Ok(Self::Cve),
            "CZK" => Ok(Self::Czk),
            "DJF" => Ok(Self::Djf),
            "DKK" => Ok(Self::Dkk),
            "DOP" => Ok(Self::Dop),
            "DZD" => Ok(Self::Dzd),
            "EGP" => Ok(Self::Egp),
            "ERN" => Ok(Self::Ern),
            "ETB" => Ok(Self::Etb),
            "EUR" => Ok(Self::Eur),
            "FJD" => Ok(Self::Fjd),
            "FKP" => Ok(Self::Fkp),
            "GBP" => Ok(Self::Gbp),
            "GEL" => Ok(Self::Gel),
            "GHS" => Ok(Self::Ghs),
            "GIP" => Ok(Self::Gip),
            "GMD" => Ok(Self::Gmd),
            "GNF" => Ok(Self::Gnf),
            "GTQ" => Ok(Self::Gtq),
            "GYD" => Ok(Self::Gyd),
            "HKD" => Ok(Self::Hkd),
            "HNL" => Ok(Self::Hnl),
            "HRK" => Ok(Self::Hrk),
            "HTG" => Ok(Self::Htg),
            "HUF" => Ok(Self::Huf),
            "IDR" => Ok(Self::Idr),
            "ILS" => Ok(Self::Ils),
            "INR" => Ok(Self::Inr),
            "IQD" => Ok(Self::Iqd),
            "IRR" => Ok(Self::Irr),
            "ISK" => Ok(Self::Isk),
            "JMD" => Ok(Self::Jmd),
            "JOD" => Ok(Self::Jod),
            "JPY" => Ok(Self::Jpy),
            "KES" => Ok(Self::Kes),
            "KGS" => Ok(Self::Kgs),
            "KHR" => Ok(Self::Khr),
            "KMF" => Ok(Self::Kmf),
            "KPW" => Ok(Self::Kpw),
            "KRW" => Ok(Self::Krw),
            "KWD" => Ok(Self::Kwd),
            "KYD" => Ok(Self::Kyd),
            "KZT" => Ok(Self::Kzt),
            "LAK" => Ok(Self::Lak),
            "LBP" => Ok(Self::Lbp),
            "LKR" => Ok(Self::Lkr),
            "LRD" => Ok(Self::Lrd),
            "LSL" => Ok(Self::Lsl),
            "LTL" => Ok(Self::Ltl),
            "LYD" => Ok(Self::Lyd),
            "MAD" => Ok(Self::Mad),
            "MDL" => Ok(Self::Mdl),
            "MGA" => Ok(Self::Mga),
            "MKD" => Ok(Self::Mkd),
            "MMK" => Ok(Self::Mmk),
            "MNT" => Ok(Self::Mnt),
            "MOP" => Ok(Self::Mop),
            "MRO" => Ok(Self::Mro),
            "MUR" => Ok(Self::Mur),
            "MVR" => Ok(Self::Mvr),
            "MWK" => Ok(Self::Mwk),
            "MXN" => Ok(Self::Mxn),
            "MXV" => Ok(Self::Mxv),
            "MYR" => Ok(Self::Myr),
            "MZN" => Ok(Self::Mzn),
            "NAD" => Ok(Self::Nad),
            "NGN" => Ok(Self::Ngn),
            "NIO" => Ok(Self::Nio),
            "NOK" => Ok(Self::Nok),
            "NPR" => Ok(Self::Npr),
            "NZD" => Ok(Self::Nzd),
            "OMR" => Ok(Self::Omr),
            "PAB" => Ok(Self::Pab),
            "PEN" => Ok(Self::Pen),
            "PGK" => Ok(Self::Pgk),
            "PHP" => Ok(Self::Php),
            "PKR" => Ok(Self::Pkr),
            "PLN" => Ok(Self::Pln),
            "PYG" => Ok(Self::Pyg),
            "QAR" => Ok(Self::Qar),
            "RON" => Ok(Self::Ron),
            "RSD" => Ok(Self::Rsd),
            "RUB" => Ok(Self::Rub),
            "RUR" => Ok(Self::Rur),
            "RWF" => Ok(Self::Rwf),
            "SAR" => Ok(Self::Sar),
            "SBD" => Ok(Self::Sbd),
            "SCR" => Ok(Self::Scr),
            "SDG" => Ok(Self::Sdg),
            "SEK" => Ok(Self::Sek),
            "SGD" => Ok(Self::Sgd),
            "SHP" => Ok(Self::Shp),
            "SLL" => Ok(Self::Sll),
            "SOS" => Ok(Self::Sos),
            "SRD" => Ok(Self::Srd),
            "SSP" => Ok(Self::Ssp),
            "STD" => Ok(Self::Std),
            "SVC" => Ok(Self::Svc),
            "SYP" => Ok(Self::Syp),
            "SZL" => Ok(Self::Szl),
            "THB" => Ok(Self::Thb),
            "TJS" => Ok(Self::Tjs),
            "TMT" => Ok(Self::Tmt),
            "TND" => Ok(Self::Tnd),
            "TOP" => Ok(Self::Top),
            "TRY" => Ok(Self::Try),
            "TTD" => Ok(Self::Ttd),
            "TWD" => Ok(Self::Twd),
            "TZS" => Ok(Self::Tzs),
            "UAH" => Ok(Self::Uah),
            "UGX" => Ok(Self::Ugx),
            "USD" => Ok(Self::Usd),
            "USN" => Ok(Self::Usn),
            "USS" => Ok(Self::Uss),
            "UYI" => Ok(Self::Uyi),
            "UYU" => Ok(Self::Uyu),
            "UZS" => Ok(Self::Uzs),
            "VEF" => Ok(Self::Vef),
            "VND" => Ok(Self::Vnd),
            "VUV" => Ok(Self::Vuv),
            "WST" => Ok(Self::Wst),
            "XAF" => Ok(Self::Xaf),
            "XAG" => Ok(Self::Xag),
            "XAU" => Ok(Self::Xau),
            "XBA" => Ok(Self::Xba),
            "XBB" => Ok(Self::Xbb),
            "XBC" => Ok(Self::Xbc),
            "XBD" => Ok(Self::Xbd),
            "XCD" => Ok(Self::Xcd),
            "XDR" => Ok(Self::Xdr),
            "XOF" => Ok(Self::Xof),
            "XPD" => Ok(Self::Xpd),
            "XPF" => Ok(Self::Xpf),
            "XPT" => Ok(Self::Xpt),
            "XSU" => Ok(Self::Xsu),
            "XTS" => Ok(Self::Xts),
            "XUA" => Ok(Self::Xua),
            "XXX" => Ok(Self::Xxx),
            "YER" => Ok(Self::Yer),
            "ZAR" => Ok(Self::Zar),
            "ZMW" => Ok(Self::Zmw),
            "ZWL" => Ok(Self::Zwl),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Waehrungscode::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Waehrungscode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Waehrungscode {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Waehrungscode {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Waehrungscode {
    const VARIANTS: &'static [Self] = Self::VARIANTS;
    const COUNT: usize = Self::COUNT;
    fn as_wire(&self) -> &'static str {
        Self::as_wire(self)
    }
    fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        Self::from_wire(s)
    }
    fn is_unknown(&self) -> bool {
        Self::is_unknown(self)
    }
}
#[cfg(feature = "versioned")]
impl crate::Bo4eStrict for Waehrungscode {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Waehrungscode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Waehrungscode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Waehrungscode::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Waehrungscode::from_wire`] on a `String` column, or check
/// [`Waehrungscode::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Waehrungscode {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
/// Lets `Vec<Waehrungscode>` bind to a `TEXT[]` column.  Only this crate can
/// provide it: the trait and the enum are both foreign to any consumer, so the
/// orphan rule rules out a downstream impl.
#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Waehrungscode {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Waehrungscode {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
