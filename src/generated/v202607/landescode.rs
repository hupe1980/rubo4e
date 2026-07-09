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
#[non_exhaustive]
pub enum Landescode {
    #[cfg_attr(feature = "serde", serde(rename = "AF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AF"))]
    Af,
    #[cfg_attr(feature = "serde", serde(rename = "AX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AX"))]
    Ax,
    #[cfg_attr(feature = "serde", serde(rename = "AL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AL"))]
    Al,
    #[cfg_attr(feature = "serde", serde(rename = "DZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DZ"))]
    Dz,
    #[cfg_attr(feature = "serde", serde(rename = "AS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AS"))]
    As,
    #[cfg_attr(feature = "serde", serde(rename = "AD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AD"))]
    Ad,
    #[cfg_attr(feature = "serde", serde(rename = "AO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AO"))]
    Ao,
    #[cfg_attr(feature = "serde", serde(rename = "AI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AI"))]
    Ai,
    #[cfg_attr(feature = "serde", serde(rename = "AQ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AQ"))]
    Aq,
    #[cfg_attr(feature = "serde", serde(rename = "AG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AG"))]
    Ag,
    #[cfg_attr(feature = "serde", serde(rename = "AR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AR"))]
    Ar,
    #[cfg_attr(feature = "serde", serde(rename = "AM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AM"))]
    Am,
    #[cfg_attr(feature = "serde", serde(rename = "AW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AW"))]
    Aw,
    #[cfg_attr(feature = "serde", serde(rename = "AU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AU"))]
    Au,
    #[cfg_attr(feature = "serde", serde(rename = "AT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AT"))]
    At,
    #[cfg_attr(feature = "serde", serde(rename = "AZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AZ"))]
    Az,
    #[cfg_attr(feature = "serde", serde(rename = "BS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BS"))]
    Bs,
    #[cfg_attr(feature = "serde", serde(rename = "BH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BH"))]
    Bh,
    #[cfg_attr(feature = "serde", serde(rename = "BD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BD"))]
    Bd,
    #[cfg_attr(feature = "serde", serde(rename = "BB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BB"))]
    Bb,
    #[cfg_attr(feature = "serde", serde(rename = "BY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BY"))]
    By,
    #[cfg_attr(feature = "serde", serde(rename = "BE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BE"))]
    Be,
    #[cfg_attr(feature = "serde", serde(rename = "BZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BZ"))]
    Bz,
    #[cfg_attr(feature = "serde", serde(rename = "BJ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BJ"))]
    Bj,
    #[cfg_attr(feature = "serde", serde(rename = "BM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BM"))]
    Bm,
    #[cfg_attr(feature = "serde", serde(rename = "BT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BT"))]
    Bt,
    #[cfg_attr(feature = "serde", serde(rename = "BO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BO"))]
    Bo,
    #[cfg_attr(feature = "serde", serde(rename = "BQ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BQ"))]
    Bq,
    #[cfg_attr(feature = "serde", serde(rename = "BA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BA"))]
    Ba,
    #[cfg_attr(feature = "serde", serde(rename = "BW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BW"))]
    Bw,
    #[cfg_attr(feature = "serde", serde(rename = "BV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BV"))]
    Bv,
    #[cfg_attr(feature = "serde", serde(rename = "BR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BR"))]
    Br,
    #[cfg_attr(feature = "serde", serde(rename = "IO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IO"))]
    Io,
    #[cfg_attr(feature = "serde", serde(rename = "BN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BN"))]
    Bn,
    #[cfg_attr(feature = "serde", serde(rename = "BG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BG"))]
    Bg,
    #[cfg_attr(feature = "serde", serde(rename = "BF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BF"))]
    Bf,
    #[cfg_attr(feature = "serde", serde(rename = "BI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BI"))]
    Bi,
    #[cfg_attr(feature = "serde", serde(rename = "KH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KH"))]
    Kh,
    #[cfg_attr(feature = "serde", serde(rename = "CM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CM"))]
    Cm,
    #[cfg_attr(feature = "serde", serde(rename = "CA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CA"))]
    Ca,
    #[cfg_attr(feature = "serde", serde(rename = "CV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CV"))]
    Cv,
    #[cfg_attr(feature = "serde", serde(rename = "KY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KY"))]
    Ky,
    #[cfg_attr(feature = "serde", serde(rename = "CF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CF"))]
    Cf,
    #[cfg_attr(feature = "serde", serde(rename = "TD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TD"))]
    Td,
    #[cfg_attr(feature = "serde", serde(rename = "CL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CL"))]
    Cl,
    #[cfg_attr(feature = "serde", serde(rename = "CN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CN"))]
    Cn,
    #[cfg_attr(feature = "serde", serde(rename = "CX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CX"))]
    Cx,
    #[cfg_attr(feature = "serde", serde(rename = "CC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CC"))]
    Cc,
    #[cfg_attr(feature = "serde", serde(rename = "CO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CO"))]
    Co,
    #[cfg_attr(feature = "serde", serde(rename = "KM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KM"))]
    Km,
    #[cfg_attr(feature = "serde", serde(rename = "CG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CG"))]
    Cg,
    #[cfg_attr(feature = "serde", serde(rename = "CD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CD"))]
    Cd,
    #[cfg_attr(feature = "serde", serde(rename = "CK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CK"))]
    Ck,
    #[cfg_attr(feature = "serde", serde(rename = "CR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CR"))]
    Cr,
    #[cfg_attr(feature = "serde", serde(rename = "CI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CI"))]
    Ci,
    #[cfg_attr(feature = "serde", serde(rename = "HR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HR"))]
    Hr,
    #[cfg_attr(feature = "serde", serde(rename = "CU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CU"))]
    Cu,
    #[cfg_attr(feature = "serde", serde(rename = "CW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CW"))]
    Cw,
    #[cfg_attr(feature = "serde", serde(rename = "CY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CY"))]
    Cy,
    #[cfg_attr(feature = "serde", serde(rename = "CZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CZ"))]
    Cz,
    #[cfg_attr(feature = "serde", serde(rename = "DK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DK"))]
    Dk,
    #[cfg_attr(feature = "serde", serde(rename = "DJ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DJ"))]
    Dj,
    #[cfg_attr(feature = "serde", serde(rename = "DM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DM"))]
    Dm,
    #[cfg_attr(feature = "serde", serde(rename = "DO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DO"))]
    Do,
    #[cfg_attr(feature = "serde", serde(rename = "EC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EC"))]
    Ec,
    #[cfg_attr(feature = "serde", serde(rename = "EG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EG"))]
    Eg,
    #[cfg_attr(feature = "serde", serde(rename = "SV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SV"))]
    Sv,
    #[cfg_attr(feature = "serde", serde(rename = "GQ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GQ"))]
    Gq,
    #[cfg_attr(feature = "serde", serde(rename = "ER"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ER"))]
    Er,
    #[cfg_attr(feature = "serde", serde(rename = "EE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EE"))]
    Ee,
    #[cfg_attr(feature = "serde", serde(rename = "ET"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ET"))]
    Et,
    #[cfg_attr(feature = "serde", serde(rename = "FK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FK"))]
    Fk,
    #[cfg_attr(feature = "serde", serde(rename = "FO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FO"))]
    Fo,
    #[cfg_attr(feature = "serde", serde(rename = "FJ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FJ"))]
    Fj,
    #[cfg_attr(feature = "serde", serde(rename = "FI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FI"))]
    Fi,
    #[cfg_attr(feature = "serde", serde(rename = "FR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FR"))]
    Fr,
    #[cfg_attr(feature = "serde", serde(rename = "GF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GF"))]
    Gf,
    #[cfg_attr(feature = "serde", serde(rename = "PF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PF"))]
    Pf,
    #[cfg_attr(feature = "serde", serde(rename = "TF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TF"))]
    Tf,
    #[cfg_attr(feature = "serde", serde(rename = "GA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GA"))]
    Ga,
    #[cfg_attr(feature = "serde", serde(rename = "GM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GM"))]
    Gm,
    #[cfg_attr(feature = "serde", serde(rename = "GE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GE"))]
    Ge,
    #[cfg_attr(feature = "serde", serde(rename = "DE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "DE"))]
    De,
    #[cfg_attr(feature = "serde", serde(rename = "GH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GH"))]
    Gh,
    #[cfg_attr(feature = "serde", serde(rename = "GI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GI"))]
    Gi,
    #[cfg_attr(feature = "serde", serde(rename = "GR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GR"))]
    Gr,
    #[cfg_attr(feature = "serde", serde(rename = "GL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GL"))]
    Gl,
    #[cfg_attr(feature = "serde", serde(rename = "GD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GD"))]
    Gd,
    #[cfg_attr(feature = "serde", serde(rename = "GP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GP"))]
    Gp,
    #[cfg_attr(feature = "serde", serde(rename = "GU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GU"))]
    Gu,
    #[cfg_attr(feature = "serde", serde(rename = "GT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GT"))]
    Gt,
    #[cfg_attr(feature = "serde", serde(rename = "GG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GG"))]
    Gg,
    #[cfg_attr(feature = "serde", serde(rename = "GN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GN"))]
    Gn,
    #[cfg_attr(feature = "serde", serde(rename = "GW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GW"))]
    Gw,
    #[cfg_attr(feature = "serde", serde(rename = "GY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GY"))]
    Gy,
    #[cfg_attr(feature = "serde", serde(rename = "HT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HT"))]
    Ht,
    #[cfg_attr(feature = "serde", serde(rename = "HM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HM"))]
    Hm,
    #[cfg_attr(feature = "serde", serde(rename = "VA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VA"))]
    Va,
    #[cfg_attr(feature = "serde", serde(rename = "HN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HN"))]
    Hn,
    #[cfg_attr(feature = "serde", serde(rename = "HK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HK"))]
    Hk,
    #[cfg_attr(feature = "serde", serde(rename = "HU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "HU"))]
    Hu,
    #[cfg_attr(feature = "serde", serde(rename = "IS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IS"))]
    Is,
    #[cfg_attr(feature = "serde", serde(rename = "IN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IN"))]
    In,
    #[cfg_attr(feature = "serde", serde(rename = "ID"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ID"))]
    Id,
    #[cfg_attr(feature = "serde", serde(rename = "IR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IR"))]
    Ir,
    #[cfg_attr(feature = "serde", serde(rename = "IQ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IQ"))]
    Iq,
    #[cfg_attr(feature = "serde", serde(rename = "IE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IE"))]
    Ie,
    #[cfg_attr(feature = "serde", serde(rename = "IM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IM"))]
    Im,
    #[cfg_attr(feature = "serde", serde(rename = "IL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IL"))]
    Il,
    #[cfg_attr(feature = "serde", serde(rename = "IT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "IT"))]
    It,
    #[cfg_attr(feature = "serde", serde(rename = "JM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JM"))]
    Jm,
    #[cfg_attr(feature = "serde", serde(rename = "JP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JP"))]
    Jp,
    #[cfg_attr(feature = "serde", serde(rename = "JE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JE"))]
    Je,
    #[cfg_attr(feature = "serde", serde(rename = "JO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "JO"))]
    Jo,
    #[cfg_attr(feature = "serde", serde(rename = "KZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KZ"))]
    Kz,
    #[cfg_attr(feature = "serde", serde(rename = "KE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KE"))]
    Ke,
    #[cfg_attr(feature = "serde", serde(rename = "KI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KI"))]
    Ki,
    #[cfg_attr(feature = "serde", serde(rename = "KP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KP"))]
    Kp,
    #[cfg_attr(feature = "serde", serde(rename = "KR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KR"))]
    Kr,
    #[cfg_attr(feature = "serde", serde(rename = "XK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "XK"))]
    Xk,
    #[cfg_attr(feature = "serde", serde(rename = "KW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KW"))]
    Kw,
    #[cfg_attr(feature = "serde", serde(rename = "KG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KG"))]
    Kg,
    #[cfg_attr(feature = "serde", serde(rename = "LA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LA"))]
    La,
    #[cfg_attr(feature = "serde", serde(rename = "LV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LV"))]
    Lv,
    #[cfg_attr(feature = "serde", serde(rename = "LB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LB"))]
    Lb,
    #[cfg_attr(feature = "serde", serde(rename = "LS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LS"))]
    Ls,
    #[cfg_attr(feature = "serde", serde(rename = "LR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LR"))]
    Lr,
    #[cfg_attr(feature = "serde", serde(rename = "LY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LY"))]
    Ly,
    #[cfg_attr(feature = "serde", serde(rename = "LI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LI"))]
    Li,
    #[cfg_attr(feature = "serde", serde(rename = "LT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LT"))]
    Lt,
    #[cfg_attr(feature = "serde", serde(rename = "LU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LU"))]
    Lu,
    #[cfg_attr(feature = "serde", serde(rename = "MO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MO"))]
    Mo,
    #[cfg_attr(feature = "serde", serde(rename = "MK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MK"))]
    Mk,
    #[cfg_attr(feature = "serde", serde(rename = "MG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MG"))]
    Mg,
    #[cfg_attr(feature = "serde", serde(rename = "MW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MW"))]
    Mw,
    #[cfg_attr(feature = "serde", serde(rename = "MY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MY"))]
    My,
    #[cfg_attr(feature = "serde", serde(rename = "MV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MV"))]
    Mv,
    #[cfg_attr(feature = "serde", serde(rename = "ML"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ML"))]
    Ml,
    #[cfg_attr(feature = "serde", serde(rename = "MT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MT"))]
    Mt,
    #[cfg_attr(feature = "serde", serde(rename = "MH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MH"))]
    Mh,
    #[cfg_attr(feature = "serde", serde(rename = "MQ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MQ"))]
    Mq,
    #[cfg_attr(feature = "serde", serde(rename = "MR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MR"))]
    Mr,
    #[cfg_attr(feature = "serde", serde(rename = "MU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MU"))]
    Mu,
    #[cfg_attr(feature = "serde", serde(rename = "YT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "YT"))]
    Yt,
    #[cfg_attr(feature = "serde", serde(rename = "MX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MX"))]
    Mx,
    #[cfg_attr(feature = "serde", serde(rename = "FM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "FM"))]
    Fm,
    #[cfg_attr(feature = "serde", serde(rename = "MD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MD"))]
    Md,
    #[cfg_attr(feature = "serde", serde(rename = "MC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MC"))]
    Mc,
    #[cfg_attr(feature = "serde", serde(rename = "MN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MN"))]
    Mn,
    #[cfg_attr(feature = "serde", serde(rename = "ME"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ME"))]
    Me,
    #[cfg_attr(feature = "serde", serde(rename = "MS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MS"))]
    Ms,
    #[cfg_attr(feature = "serde", serde(rename = "MA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MA"))]
    Ma,
    #[cfg_attr(feature = "serde", serde(rename = "MZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MZ"))]
    Mz,
    #[cfg_attr(feature = "serde", serde(rename = "MM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MM"))]
    Mm,
    #[cfg_attr(feature = "serde", serde(rename = "NA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NA"))]
    Na,
    #[cfg_attr(feature = "serde", serde(rename = "NR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NR"))]
    Nr,
    #[cfg_attr(feature = "serde", serde(rename = "NP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NP"))]
    Np,
    #[cfg_attr(feature = "serde", serde(rename = "NL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NL"))]
    Nl,
    #[cfg_attr(feature = "serde", serde(rename = "NC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NC"))]
    Nc,
    #[cfg_attr(feature = "serde", serde(rename = "NZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NZ"))]
    Nz,
    #[cfg_attr(feature = "serde", serde(rename = "NI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NI"))]
    Ni,
    #[cfg_attr(feature = "serde", serde(rename = "NE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NE"))]
    Ne,
    #[cfg_attr(feature = "serde", serde(rename = "NG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NG"))]
    Ng,
    #[cfg_attr(feature = "serde", serde(rename = "NU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NU"))]
    Nu,
    #[cfg_attr(feature = "serde", serde(rename = "NF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NF"))]
    Nf,
    #[cfg_attr(feature = "serde", serde(rename = "MP"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MP"))]
    Mp,
    #[cfg_attr(feature = "serde", serde(rename = "NO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "NO"))]
    No,
    #[cfg_attr(feature = "serde", serde(rename = "OM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "OM"))]
    Om,
    #[cfg_attr(feature = "serde", serde(rename = "PK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PK"))]
    Pk,
    #[cfg_attr(feature = "serde", serde(rename = "PW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PW"))]
    Pw,
    #[cfg_attr(feature = "serde", serde(rename = "PS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PS"))]
    Ps,
    #[cfg_attr(feature = "serde", serde(rename = "PA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PA"))]
    Pa,
    #[cfg_attr(feature = "serde", serde(rename = "PG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PG"))]
    Pg,
    #[cfg_attr(feature = "serde", serde(rename = "PY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PY"))]
    Py,
    #[cfg_attr(feature = "serde", serde(rename = "PE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PE"))]
    Pe,
    #[cfg_attr(feature = "serde", serde(rename = "PH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PH"))]
    Ph,
    #[cfg_attr(feature = "serde", serde(rename = "PN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PN"))]
    Pn,
    #[cfg_attr(feature = "serde", serde(rename = "PL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PL"))]
    Pl,
    #[cfg_attr(feature = "serde", serde(rename = "PT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PT"))]
    Pt,
    #[cfg_attr(feature = "serde", serde(rename = "PR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PR"))]
    Pr,
    #[cfg_attr(feature = "serde", serde(rename = "QA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "QA"))]
    Qa,
    #[cfg_attr(feature = "serde", serde(rename = "RE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RE"))]
    Re,
    #[cfg_attr(feature = "serde", serde(rename = "RO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RO"))]
    Ro,
    #[cfg_attr(feature = "serde", serde(rename = "RU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RU"))]
    Ru,
    #[cfg_attr(feature = "serde", serde(rename = "RW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RW"))]
    Rw,
    #[cfg_attr(feature = "serde", serde(rename = "BL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "BL"))]
    Bl,
    #[cfg_attr(feature = "serde", serde(rename = "SH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SH"))]
    Sh,
    #[cfg_attr(feature = "serde", serde(rename = "KN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "KN"))]
    Kn,
    #[cfg_attr(feature = "serde", serde(rename = "LC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LC"))]
    Lc,
    #[cfg_attr(feature = "serde", serde(rename = "MF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "MF"))]
    Mf,
    #[cfg_attr(feature = "serde", serde(rename = "PM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "PM"))]
    Pm,
    #[cfg_attr(feature = "serde", serde(rename = "VC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VC"))]
    Vc,
    #[cfg_attr(feature = "serde", serde(rename = "WS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WS"))]
    Ws,
    #[cfg_attr(feature = "serde", serde(rename = "SM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SM"))]
    Sm,
    #[cfg_attr(feature = "serde", serde(rename = "ST"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ST"))]
    St,
    #[cfg_attr(feature = "serde", serde(rename = "SA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SA"))]
    Sa,
    #[cfg_attr(feature = "serde", serde(rename = "SN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SN"))]
    Sn,
    #[cfg_attr(feature = "serde", serde(rename = "RS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "RS"))]
    Rs,
    #[cfg_attr(feature = "serde", serde(rename = "SC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SC"))]
    Sc,
    #[cfg_attr(feature = "serde", serde(rename = "SL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SL"))]
    Sl,
    #[cfg_attr(feature = "serde", serde(rename = "SG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SG"))]
    Sg,
    #[cfg_attr(feature = "serde", serde(rename = "SX"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SX"))]
    Sx,
    #[cfg_attr(feature = "serde", serde(rename = "SK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SK"))]
    Sk,
    #[cfg_attr(feature = "serde", serde(rename = "SI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SI"))]
    Si,
    #[cfg_attr(feature = "serde", serde(rename = "SB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SB"))]
    Sb,
    #[cfg_attr(feature = "serde", serde(rename = "SO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SO"))]
    So,
    #[cfg_attr(feature = "serde", serde(rename = "ZA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZA"))]
    Za,
    #[cfg_attr(feature = "serde", serde(rename = "GS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GS"))]
    Gs,
    #[cfg_attr(feature = "serde", serde(rename = "SS"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SS"))]
    Ss,
    #[cfg_attr(feature = "serde", serde(rename = "ES"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ES"))]
    Es,
    #[cfg_attr(feature = "serde", serde(rename = "LK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "LK"))]
    Lk,
    #[cfg_attr(feature = "serde", serde(rename = "SD"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SD"))]
    Sd,
    #[cfg_attr(feature = "serde", serde(rename = "SR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SR"))]
    Sr,
    #[cfg_attr(feature = "serde", serde(rename = "SJ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SJ"))]
    Sj,
    #[cfg_attr(feature = "serde", serde(rename = "SZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SZ"))]
    Sz,
    #[cfg_attr(feature = "serde", serde(rename = "SE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SE"))]
    Se,
    #[cfg_attr(feature = "serde", serde(rename = "CH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "CH"))]
    Ch,
    #[cfg_attr(feature = "serde", serde(rename = "SY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "SY"))]
    Sy,
    #[cfg_attr(feature = "serde", serde(rename = "TW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TW"))]
    Tw,
    #[cfg_attr(feature = "serde", serde(rename = "TJ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TJ"))]
    Tj,
    #[cfg_attr(feature = "serde", serde(rename = "TZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TZ"))]
    Tz,
    #[cfg_attr(feature = "serde", serde(rename = "TH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TH"))]
    Th,
    #[cfg_attr(feature = "serde", serde(rename = "TL"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TL"))]
    Tl,
    #[cfg_attr(feature = "serde", serde(rename = "TG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TG"))]
    Tg,
    #[cfg_attr(feature = "serde", serde(rename = "TK"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TK"))]
    Tk,
    #[cfg_attr(feature = "serde", serde(rename = "TO"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TO"))]
    To,
    #[cfg_attr(feature = "serde", serde(rename = "TT"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TT"))]
    Tt,
    #[cfg_attr(feature = "serde", serde(rename = "TN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TN"))]
    Tn,
    #[cfg_attr(feature = "serde", serde(rename = "TR"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TR"))]
    Tr,
    #[cfg_attr(feature = "serde", serde(rename = "TM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TM"))]
    Tm,
    #[cfg_attr(feature = "serde", serde(rename = "TC"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TC"))]
    Tc,
    #[cfg_attr(feature = "serde", serde(rename = "TV"))]
    #[cfg_attr(feature = "strum", strum(serialize = "TV"))]
    Tv,
    #[cfg_attr(feature = "serde", serde(rename = "UG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UG"))]
    Ug,
    #[cfg_attr(feature = "serde", serde(rename = "UA"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UA"))]
    Ua,
    #[cfg_attr(feature = "serde", serde(rename = "AE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "AE"))]
    Ae,
    #[cfg_attr(feature = "serde", serde(rename = "GB"))]
    #[cfg_attr(feature = "strum", strum(serialize = "GB"))]
    Gb,
    #[cfg_attr(feature = "serde", serde(rename = "US"))]
    #[cfg_attr(feature = "strum", strum(serialize = "US"))]
    Us,
    #[cfg_attr(feature = "serde", serde(rename = "UM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UM"))]
    Um,
    #[cfg_attr(feature = "serde", serde(rename = "UY"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UY"))]
    Uy,
    #[cfg_attr(feature = "serde", serde(rename = "UZ"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UZ"))]
    Uz,
    #[cfg_attr(feature = "serde", serde(rename = "VU"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VU"))]
    Vu,
    #[cfg_attr(feature = "serde", serde(rename = "VE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VE"))]
    Ve,
    #[cfg_attr(feature = "serde", serde(rename = "VN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VN"))]
    Vn,
    #[cfg_attr(feature = "serde", serde(rename = "VG"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VG"))]
    Vg,
    #[cfg_attr(feature = "serde", serde(rename = "VI"))]
    #[cfg_attr(feature = "strum", strum(serialize = "VI"))]
    Vi,
    #[cfg_attr(feature = "serde", serde(rename = "WF"))]
    #[cfg_attr(feature = "strum", strum(serialize = "WF"))]
    Wf,
    #[cfg_attr(feature = "serde", serde(rename = "EH"))]
    #[cfg_attr(feature = "strum", strum(serialize = "EH"))]
    Eh,
    #[cfg_attr(feature = "serde", serde(rename = "YE"))]
    #[cfg_attr(feature = "strum", strum(serialize = "YE"))]
    Ye,
    #[cfg_attr(feature = "serde", serde(rename = "ZM"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZM"))]
    Zm,
    #[cfg_attr(feature = "serde", serde(rename = "ZW"))]
    #[cfg_attr(feature = "strum", strum(serialize = "ZW"))]
    Zw,
    /// Unknown or future variant — produced when deserializing a value
    /// that is not yet known to this version of the library.
    #[cfg_attr(feature = "serde", serde(other, rename = "UNKNOWN"))]
    #[cfg_attr(feature = "strum", strum(serialize = "UNKNOWN"))]
    Unknown,
}
impl Landescode {
    /// Returns an iterator over all **known** variants of `Landescode`.
    ///
    /// Unlike [`strum::IntoEnumIterator`] which includes the [`Landescode::Unknown`]
    /// catch-all, this method yields only variants that correspond to values defined
    /// in the BO4E schema.  Use this when building dropdowns, lookup tables, or
    /// generating reports that should only include valid schema values.
    ///
    /// # Example
    /// ```rust,ignore
    /// for v in Landescode::iter_known() {
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
impl sqlx::Type<sqlx::Postgres> for Landescode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Strum fast path: `AsRef<str>` returns the canonical string without a
/// `serde_json::Value` intermediate, saving an allocation per encode (M-07).
#[cfg(all(feature = "sqlx", feature = "json", feature = "strum"))]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Landescode {
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
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Landescode {
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
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Landescode {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <String as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        serde_json::from_value(serde_json::Value::String(s))
            .map_err(|e| Box::new(e) as sqlx::error::BoxDynError)
    }
}
#[cfg(all(test, feature = "strum"))]
impl proptest::arbitrary::Arbitrary for Landescode {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        use strum::IntoEnumIterator as _;
        let variants: Vec<Self> = Self::iter().collect();
        proptest::sample::select(variants).boxed()
    }
}
