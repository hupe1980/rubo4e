#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(
        strum::Display,
        strum::EnumString,
        strum::EnumIter,
        strum::IntoStaticStr,
        strum::AsRefStr
    )
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
    /// Returns an iterator over all **known** variants of `Waehrungscode`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Waehrungscode::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Waehrungscode::iter_known() {
    ///     println!("{v}");
    /// }
    /// ```
    #[cfg(feature = "strum")]
    pub fn iter_known() -> impl Iterator<Item = Self> {
        use strum::IntoEnumIterator as _;
        Self::iter().filter(|v| !matches!(v, Self::Unknown))
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl sqlx::Type<sqlx::Postgres> for Waehrungscode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Waehrungscode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_ref();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Fallback when `strum` is not active: serialize via `serde_json`.
#[cfg(all(feature = "sqlx", feature = "json", not(feature = "strum")))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Waehrungscode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s = serde_json::to_value(self)?
            .as_str()
            .ok_or("enum variant did not serialize to a JSON string")?
            .to_owned();
        <String as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
#[cfg(all(feature = "sqlx", feature = "json"))]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Waehrungscode {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Waehrungscode {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
