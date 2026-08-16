#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumString, strum::EnumIter, strum::IntoStaticStr)
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
    /// All variants defined by the BO4E schema, in declaration order.
    ///
    /// Excludes the forward-compatibility [`Landescode::Unknown`] catch-all, so this
    /// is exactly the set of values that appear on the wire.  Available **without**
    /// the `strum` feature — use it to drift-guard SQL `CHECK` lists and mappings.
    pub const VARIANTS: &'static [Self] = &[
        Self::Af,
        Self::Ax,
        Self::Al,
        Self::Dz,
        Self::As,
        Self::Ad,
        Self::Ao,
        Self::Ai,
        Self::Aq,
        Self::Ag,
        Self::Ar,
        Self::Am,
        Self::Aw,
        Self::Au,
        Self::At,
        Self::Az,
        Self::Bs,
        Self::Bh,
        Self::Bd,
        Self::Bb,
        Self::By,
        Self::Be,
        Self::Bz,
        Self::Bj,
        Self::Bm,
        Self::Bt,
        Self::Bo,
        Self::Bq,
        Self::Ba,
        Self::Bw,
        Self::Bv,
        Self::Br,
        Self::Io,
        Self::Bn,
        Self::Bg,
        Self::Bf,
        Self::Bi,
        Self::Kh,
        Self::Cm,
        Self::Ca,
        Self::Cv,
        Self::Ky,
        Self::Cf,
        Self::Td,
        Self::Cl,
        Self::Cn,
        Self::Cx,
        Self::Cc,
        Self::Co,
        Self::Km,
        Self::Cg,
        Self::Cd,
        Self::Ck,
        Self::Cr,
        Self::Ci,
        Self::Hr,
        Self::Cu,
        Self::Cw,
        Self::Cy,
        Self::Cz,
        Self::Dk,
        Self::Dj,
        Self::Dm,
        Self::Do,
        Self::Ec,
        Self::Eg,
        Self::Sv,
        Self::Gq,
        Self::Er,
        Self::Ee,
        Self::Et,
        Self::Fk,
        Self::Fo,
        Self::Fj,
        Self::Fi,
        Self::Fr,
        Self::Gf,
        Self::Pf,
        Self::Tf,
        Self::Ga,
        Self::Gm,
        Self::Ge,
        Self::De,
        Self::Gh,
        Self::Gi,
        Self::Gr,
        Self::Gl,
        Self::Gd,
        Self::Gp,
        Self::Gu,
        Self::Gt,
        Self::Gg,
        Self::Gn,
        Self::Gw,
        Self::Gy,
        Self::Ht,
        Self::Hm,
        Self::Va,
        Self::Hn,
        Self::Hk,
        Self::Hu,
        Self::Is,
        Self::In,
        Self::Id,
        Self::Ir,
        Self::Iq,
        Self::Ie,
        Self::Im,
        Self::Il,
        Self::It,
        Self::Jm,
        Self::Jp,
        Self::Je,
        Self::Jo,
        Self::Kz,
        Self::Ke,
        Self::Ki,
        Self::Kp,
        Self::Kr,
        Self::Xk,
        Self::Kw,
        Self::Kg,
        Self::La,
        Self::Lv,
        Self::Lb,
        Self::Ls,
        Self::Lr,
        Self::Ly,
        Self::Li,
        Self::Lt,
        Self::Lu,
        Self::Mo,
        Self::Mk,
        Self::Mg,
        Self::Mw,
        Self::My,
        Self::Mv,
        Self::Ml,
        Self::Mt,
        Self::Mh,
        Self::Mq,
        Self::Mr,
        Self::Mu,
        Self::Yt,
        Self::Mx,
        Self::Fm,
        Self::Md,
        Self::Mc,
        Self::Mn,
        Self::Me,
        Self::Ms,
        Self::Ma,
        Self::Mz,
        Self::Mm,
        Self::Na,
        Self::Nr,
        Self::Np,
        Self::Nl,
        Self::Nc,
        Self::Nz,
        Self::Ni,
        Self::Ne,
        Self::Ng,
        Self::Nu,
        Self::Nf,
        Self::Mp,
        Self::No,
        Self::Om,
        Self::Pk,
        Self::Pw,
        Self::Ps,
        Self::Pa,
        Self::Pg,
        Self::Py,
        Self::Pe,
        Self::Ph,
        Self::Pn,
        Self::Pl,
        Self::Pt,
        Self::Pr,
        Self::Qa,
        Self::Re,
        Self::Ro,
        Self::Ru,
        Self::Rw,
        Self::Bl,
        Self::Sh,
        Self::Kn,
        Self::Lc,
        Self::Mf,
        Self::Pm,
        Self::Vc,
        Self::Ws,
        Self::Sm,
        Self::St,
        Self::Sa,
        Self::Sn,
        Self::Rs,
        Self::Sc,
        Self::Sl,
        Self::Sg,
        Self::Sx,
        Self::Sk,
        Self::Si,
        Self::Sb,
        Self::So,
        Self::Za,
        Self::Gs,
        Self::Ss,
        Self::Es,
        Self::Lk,
        Self::Sd,
        Self::Sr,
        Self::Sj,
        Self::Sz,
        Self::Se,
        Self::Ch,
        Self::Sy,
        Self::Tw,
        Self::Tj,
        Self::Tz,
        Self::Th,
        Self::Tl,
        Self::Tg,
        Self::Tk,
        Self::To,
        Self::Tt,
        Self::Tn,
        Self::Tr,
        Self::Tm,
        Self::Tc,
        Self::Tv,
        Self::Ug,
        Self::Ua,
        Self::Ae,
        Self::Gb,
        Self::Us,
        Self::Um,
        Self::Uy,
        Self::Uz,
        Self::Vu,
        Self::Ve,
        Self::Vn,
        Self::Vg,
        Self::Vi,
        Self::Wf,
        Self::Eh,
        Self::Ye,
        Self::Zm,
        Self::Zw,
    ];
    /// Number of schema-defined variants (equal to `VARIANTS.len()`), excluding the
    /// [`Landescode::Unknown`] catch-all.  Stable for this schema version.
    pub const COUNT: usize = Self::VARIANTS.len();
    /// Returns an iterator over all **known** variants of `Landescode`.
    ///
    /// Yields only variants that correspond to values defined in the BO4E schema
    /// (i.e. [`Self::VARIANTS`]), never the [`Landescode::Unknown`] catch-all.
    /// Available **without** the `strum` feature.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Landescode;
    /// // Never yields the `Unknown` catch-all, so the count matches `COUNT`.
    /// assert_eq!(Landescode::iter_known().count(), Landescode::COUNT);
    /// assert!(Landescode::iter_known().all(|v| v.is_known()));
    /// ```
    pub fn iter_known() -> impl Iterator<Item = Self> + Clone {
        Self::VARIANTS.iter().copied()
    }
    /// Returns the canonical BO4E wire string (SCREAMING_SNAKE_CASE) for this value.
    ///
    /// [`Landescode::Unknown`] renders as `"UNKNOWN"`, matching its serialized form.
    pub const fn as_wire(&self) -> &'static str {
        match self {
            Self::Af => "AF",
            Self::Ax => "AX",
            Self::Al => "AL",
            Self::Dz => "DZ",
            Self::As => "AS",
            Self::Ad => "AD",
            Self::Ao => "AO",
            Self::Ai => "AI",
            Self::Aq => "AQ",
            Self::Ag => "AG",
            Self::Ar => "AR",
            Self::Am => "AM",
            Self::Aw => "AW",
            Self::Au => "AU",
            Self::At => "AT",
            Self::Az => "AZ",
            Self::Bs => "BS",
            Self::Bh => "BH",
            Self::Bd => "BD",
            Self::Bb => "BB",
            Self::By => "BY",
            Self::Be => "BE",
            Self::Bz => "BZ",
            Self::Bj => "BJ",
            Self::Bm => "BM",
            Self::Bt => "BT",
            Self::Bo => "BO",
            Self::Bq => "BQ",
            Self::Ba => "BA",
            Self::Bw => "BW",
            Self::Bv => "BV",
            Self::Br => "BR",
            Self::Io => "IO",
            Self::Bn => "BN",
            Self::Bg => "BG",
            Self::Bf => "BF",
            Self::Bi => "BI",
            Self::Kh => "KH",
            Self::Cm => "CM",
            Self::Ca => "CA",
            Self::Cv => "CV",
            Self::Ky => "KY",
            Self::Cf => "CF",
            Self::Td => "TD",
            Self::Cl => "CL",
            Self::Cn => "CN",
            Self::Cx => "CX",
            Self::Cc => "CC",
            Self::Co => "CO",
            Self::Km => "KM",
            Self::Cg => "CG",
            Self::Cd => "CD",
            Self::Ck => "CK",
            Self::Cr => "CR",
            Self::Ci => "CI",
            Self::Hr => "HR",
            Self::Cu => "CU",
            Self::Cw => "CW",
            Self::Cy => "CY",
            Self::Cz => "CZ",
            Self::Dk => "DK",
            Self::Dj => "DJ",
            Self::Dm => "DM",
            Self::Do => "DO",
            Self::Ec => "EC",
            Self::Eg => "EG",
            Self::Sv => "SV",
            Self::Gq => "GQ",
            Self::Er => "ER",
            Self::Ee => "EE",
            Self::Et => "ET",
            Self::Fk => "FK",
            Self::Fo => "FO",
            Self::Fj => "FJ",
            Self::Fi => "FI",
            Self::Fr => "FR",
            Self::Gf => "GF",
            Self::Pf => "PF",
            Self::Tf => "TF",
            Self::Ga => "GA",
            Self::Gm => "GM",
            Self::Ge => "GE",
            Self::De => "DE",
            Self::Gh => "GH",
            Self::Gi => "GI",
            Self::Gr => "GR",
            Self::Gl => "GL",
            Self::Gd => "GD",
            Self::Gp => "GP",
            Self::Gu => "GU",
            Self::Gt => "GT",
            Self::Gg => "GG",
            Self::Gn => "GN",
            Self::Gw => "GW",
            Self::Gy => "GY",
            Self::Ht => "HT",
            Self::Hm => "HM",
            Self::Va => "VA",
            Self::Hn => "HN",
            Self::Hk => "HK",
            Self::Hu => "HU",
            Self::Is => "IS",
            Self::In => "IN",
            Self::Id => "ID",
            Self::Ir => "IR",
            Self::Iq => "IQ",
            Self::Ie => "IE",
            Self::Im => "IM",
            Self::Il => "IL",
            Self::It => "IT",
            Self::Jm => "JM",
            Self::Jp => "JP",
            Self::Je => "JE",
            Self::Jo => "JO",
            Self::Kz => "KZ",
            Self::Ke => "KE",
            Self::Ki => "KI",
            Self::Kp => "KP",
            Self::Kr => "KR",
            Self::Xk => "XK",
            Self::Kw => "KW",
            Self::Kg => "KG",
            Self::La => "LA",
            Self::Lv => "LV",
            Self::Lb => "LB",
            Self::Ls => "LS",
            Self::Lr => "LR",
            Self::Ly => "LY",
            Self::Li => "LI",
            Self::Lt => "LT",
            Self::Lu => "LU",
            Self::Mo => "MO",
            Self::Mk => "MK",
            Self::Mg => "MG",
            Self::Mw => "MW",
            Self::My => "MY",
            Self::Mv => "MV",
            Self::Ml => "ML",
            Self::Mt => "MT",
            Self::Mh => "MH",
            Self::Mq => "MQ",
            Self::Mr => "MR",
            Self::Mu => "MU",
            Self::Yt => "YT",
            Self::Mx => "MX",
            Self::Fm => "FM",
            Self::Md => "MD",
            Self::Mc => "MC",
            Self::Mn => "MN",
            Self::Me => "ME",
            Self::Ms => "MS",
            Self::Ma => "MA",
            Self::Mz => "MZ",
            Self::Mm => "MM",
            Self::Na => "NA",
            Self::Nr => "NR",
            Self::Np => "NP",
            Self::Nl => "NL",
            Self::Nc => "NC",
            Self::Nz => "NZ",
            Self::Ni => "NI",
            Self::Ne => "NE",
            Self::Ng => "NG",
            Self::Nu => "NU",
            Self::Nf => "NF",
            Self::Mp => "MP",
            Self::No => "NO",
            Self::Om => "OM",
            Self::Pk => "PK",
            Self::Pw => "PW",
            Self::Ps => "PS",
            Self::Pa => "PA",
            Self::Pg => "PG",
            Self::Py => "PY",
            Self::Pe => "PE",
            Self::Ph => "PH",
            Self::Pn => "PN",
            Self::Pl => "PL",
            Self::Pt => "PT",
            Self::Pr => "PR",
            Self::Qa => "QA",
            Self::Re => "RE",
            Self::Ro => "RO",
            Self::Ru => "RU",
            Self::Rw => "RW",
            Self::Bl => "BL",
            Self::Sh => "SH",
            Self::Kn => "KN",
            Self::Lc => "LC",
            Self::Mf => "MF",
            Self::Pm => "PM",
            Self::Vc => "VC",
            Self::Ws => "WS",
            Self::Sm => "SM",
            Self::St => "ST",
            Self::Sa => "SA",
            Self::Sn => "SN",
            Self::Rs => "RS",
            Self::Sc => "SC",
            Self::Sl => "SL",
            Self::Sg => "SG",
            Self::Sx => "SX",
            Self::Sk => "SK",
            Self::Si => "SI",
            Self::Sb => "SB",
            Self::So => "SO",
            Self::Za => "ZA",
            Self::Gs => "GS",
            Self::Ss => "SS",
            Self::Es => "ES",
            Self::Lk => "LK",
            Self::Sd => "SD",
            Self::Sr => "SR",
            Self::Sj => "SJ",
            Self::Sz => "SZ",
            Self::Se => "SE",
            Self::Ch => "CH",
            Self::Sy => "SY",
            Self::Tw => "TW",
            Self::Tj => "TJ",
            Self::Tz => "TZ",
            Self::Th => "TH",
            Self::Tl => "TL",
            Self::Tg => "TG",
            Self::Tk => "TK",
            Self::To => "TO",
            Self::Tt => "TT",
            Self::Tn => "TN",
            Self::Tr => "TR",
            Self::Tm => "TM",
            Self::Tc => "TC",
            Self::Tv => "TV",
            Self::Ug => "UG",
            Self::Ua => "UA",
            Self::Ae => "AE",
            Self::Gb => "GB",
            Self::Us => "US",
            Self::Um => "UM",
            Self::Uy => "UY",
            Self::Uz => "UZ",
            Self::Vu => "VU",
            Self::Ve => "VE",
            Self::Vn => "VN",
            Self::Vg => "VG",
            Self::Vi => "VI",
            Self::Wf => "WF",
            Self::Eh => "EH",
            Self::Ye => "YE",
            Self::Zm => "ZM",
            Self::Zw => "ZW",
            Self::Unknown => "UNKNOWN",
        }
    }
    /// **Strictly** parses a BO4E wire string into a known variant.
    ///
    /// Unlike the lenient `serde` / [`FromStr`](std::str::FromStr) path — which maps
    /// any unrecognized value (a typo, a legacy code, or a value from a newer schema)
    /// to [`Landescode::Unknown`] — this returns
    /// [`Err`](crate::error::UnknownVariant) for values not defined in this schema
    /// version, including the literal `"UNKNOWN"`.  Use it at the ingest boundary to
    /// reject bad values instead of silently degrading them.
    ///
    /// # Example
    /// ```
    /// # use rubo4e::current::Landescode;
    /// /// assert_eq!(Landescode::from_wire("AF"), Ok(Landescode::Af));
    /// assert!(Landescode::from_wire("NOT_A_REAL_VALUE").is_err());
    /// // …including the `Unknown` catch-all's own wire spelling:
    /// assert!(Landescode::from_wire("UNKNOWN").is_err());
    /// ```
    pub fn from_wire(s: &str) -> Result<Self, crate::error::UnknownVariant> {
        match s {
            "AF" => Ok(Self::Af),
            "AX" => Ok(Self::Ax),
            "AL" => Ok(Self::Al),
            "DZ" => Ok(Self::Dz),
            "AS" => Ok(Self::As),
            "AD" => Ok(Self::Ad),
            "AO" => Ok(Self::Ao),
            "AI" => Ok(Self::Ai),
            "AQ" => Ok(Self::Aq),
            "AG" => Ok(Self::Ag),
            "AR" => Ok(Self::Ar),
            "AM" => Ok(Self::Am),
            "AW" => Ok(Self::Aw),
            "AU" => Ok(Self::Au),
            "AT" => Ok(Self::At),
            "AZ" => Ok(Self::Az),
            "BS" => Ok(Self::Bs),
            "BH" => Ok(Self::Bh),
            "BD" => Ok(Self::Bd),
            "BB" => Ok(Self::Bb),
            "BY" => Ok(Self::By),
            "BE" => Ok(Self::Be),
            "BZ" => Ok(Self::Bz),
            "BJ" => Ok(Self::Bj),
            "BM" => Ok(Self::Bm),
            "BT" => Ok(Self::Bt),
            "BO" => Ok(Self::Bo),
            "BQ" => Ok(Self::Bq),
            "BA" => Ok(Self::Ba),
            "BW" => Ok(Self::Bw),
            "BV" => Ok(Self::Bv),
            "BR" => Ok(Self::Br),
            "IO" => Ok(Self::Io),
            "BN" => Ok(Self::Bn),
            "BG" => Ok(Self::Bg),
            "BF" => Ok(Self::Bf),
            "BI" => Ok(Self::Bi),
            "KH" => Ok(Self::Kh),
            "CM" => Ok(Self::Cm),
            "CA" => Ok(Self::Ca),
            "CV" => Ok(Self::Cv),
            "KY" => Ok(Self::Ky),
            "CF" => Ok(Self::Cf),
            "TD" => Ok(Self::Td),
            "CL" => Ok(Self::Cl),
            "CN" => Ok(Self::Cn),
            "CX" => Ok(Self::Cx),
            "CC" => Ok(Self::Cc),
            "CO" => Ok(Self::Co),
            "KM" => Ok(Self::Km),
            "CG" => Ok(Self::Cg),
            "CD" => Ok(Self::Cd),
            "CK" => Ok(Self::Ck),
            "CR" => Ok(Self::Cr),
            "CI" => Ok(Self::Ci),
            "HR" => Ok(Self::Hr),
            "CU" => Ok(Self::Cu),
            "CW" => Ok(Self::Cw),
            "CY" => Ok(Self::Cy),
            "CZ" => Ok(Self::Cz),
            "DK" => Ok(Self::Dk),
            "DJ" => Ok(Self::Dj),
            "DM" => Ok(Self::Dm),
            "DO" => Ok(Self::Do),
            "EC" => Ok(Self::Ec),
            "EG" => Ok(Self::Eg),
            "SV" => Ok(Self::Sv),
            "GQ" => Ok(Self::Gq),
            "ER" => Ok(Self::Er),
            "EE" => Ok(Self::Ee),
            "ET" => Ok(Self::Et),
            "FK" => Ok(Self::Fk),
            "FO" => Ok(Self::Fo),
            "FJ" => Ok(Self::Fj),
            "FI" => Ok(Self::Fi),
            "FR" => Ok(Self::Fr),
            "GF" => Ok(Self::Gf),
            "PF" => Ok(Self::Pf),
            "TF" => Ok(Self::Tf),
            "GA" => Ok(Self::Ga),
            "GM" => Ok(Self::Gm),
            "GE" => Ok(Self::Ge),
            "DE" => Ok(Self::De),
            "GH" => Ok(Self::Gh),
            "GI" => Ok(Self::Gi),
            "GR" => Ok(Self::Gr),
            "GL" => Ok(Self::Gl),
            "GD" => Ok(Self::Gd),
            "GP" => Ok(Self::Gp),
            "GU" => Ok(Self::Gu),
            "GT" => Ok(Self::Gt),
            "GG" => Ok(Self::Gg),
            "GN" => Ok(Self::Gn),
            "GW" => Ok(Self::Gw),
            "GY" => Ok(Self::Gy),
            "HT" => Ok(Self::Ht),
            "HM" => Ok(Self::Hm),
            "VA" => Ok(Self::Va),
            "HN" => Ok(Self::Hn),
            "HK" => Ok(Self::Hk),
            "HU" => Ok(Self::Hu),
            "IS" => Ok(Self::Is),
            "IN" => Ok(Self::In),
            "ID" => Ok(Self::Id),
            "IR" => Ok(Self::Ir),
            "IQ" => Ok(Self::Iq),
            "IE" => Ok(Self::Ie),
            "IM" => Ok(Self::Im),
            "IL" => Ok(Self::Il),
            "IT" => Ok(Self::It),
            "JM" => Ok(Self::Jm),
            "JP" => Ok(Self::Jp),
            "JE" => Ok(Self::Je),
            "JO" => Ok(Self::Jo),
            "KZ" => Ok(Self::Kz),
            "KE" => Ok(Self::Ke),
            "KI" => Ok(Self::Ki),
            "KP" => Ok(Self::Kp),
            "KR" => Ok(Self::Kr),
            "XK" => Ok(Self::Xk),
            "KW" => Ok(Self::Kw),
            "KG" => Ok(Self::Kg),
            "LA" => Ok(Self::La),
            "LV" => Ok(Self::Lv),
            "LB" => Ok(Self::Lb),
            "LS" => Ok(Self::Ls),
            "LR" => Ok(Self::Lr),
            "LY" => Ok(Self::Ly),
            "LI" => Ok(Self::Li),
            "LT" => Ok(Self::Lt),
            "LU" => Ok(Self::Lu),
            "MO" => Ok(Self::Mo),
            "MK" => Ok(Self::Mk),
            "MG" => Ok(Self::Mg),
            "MW" => Ok(Self::Mw),
            "MY" => Ok(Self::My),
            "MV" => Ok(Self::Mv),
            "ML" => Ok(Self::Ml),
            "MT" => Ok(Self::Mt),
            "MH" => Ok(Self::Mh),
            "MQ" => Ok(Self::Mq),
            "MR" => Ok(Self::Mr),
            "MU" => Ok(Self::Mu),
            "YT" => Ok(Self::Yt),
            "MX" => Ok(Self::Mx),
            "FM" => Ok(Self::Fm),
            "MD" => Ok(Self::Md),
            "MC" => Ok(Self::Mc),
            "MN" => Ok(Self::Mn),
            "ME" => Ok(Self::Me),
            "MS" => Ok(Self::Ms),
            "MA" => Ok(Self::Ma),
            "MZ" => Ok(Self::Mz),
            "MM" => Ok(Self::Mm),
            "NA" => Ok(Self::Na),
            "NR" => Ok(Self::Nr),
            "NP" => Ok(Self::Np),
            "NL" => Ok(Self::Nl),
            "NC" => Ok(Self::Nc),
            "NZ" => Ok(Self::Nz),
            "NI" => Ok(Self::Ni),
            "NE" => Ok(Self::Ne),
            "NG" => Ok(Self::Ng),
            "NU" => Ok(Self::Nu),
            "NF" => Ok(Self::Nf),
            "MP" => Ok(Self::Mp),
            "NO" => Ok(Self::No),
            "OM" => Ok(Self::Om),
            "PK" => Ok(Self::Pk),
            "PW" => Ok(Self::Pw),
            "PS" => Ok(Self::Ps),
            "PA" => Ok(Self::Pa),
            "PG" => Ok(Self::Pg),
            "PY" => Ok(Self::Py),
            "PE" => Ok(Self::Pe),
            "PH" => Ok(Self::Ph),
            "PN" => Ok(Self::Pn),
            "PL" => Ok(Self::Pl),
            "PT" => Ok(Self::Pt),
            "PR" => Ok(Self::Pr),
            "QA" => Ok(Self::Qa),
            "RE" => Ok(Self::Re),
            "RO" => Ok(Self::Ro),
            "RU" => Ok(Self::Ru),
            "RW" => Ok(Self::Rw),
            "BL" => Ok(Self::Bl),
            "SH" => Ok(Self::Sh),
            "KN" => Ok(Self::Kn),
            "LC" => Ok(Self::Lc),
            "MF" => Ok(Self::Mf),
            "PM" => Ok(Self::Pm),
            "VC" => Ok(Self::Vc),
            "WS" => Ok(Self::Ws),
            "SM" => Ok(Self::Sm),
            "ST" => Ok(Self::St),
            "SA" => Ok(Self::Sa),
            "SN" => Ok(Self::Sn),
            "RS" => Ok(Self::Rs),
            "SC" => Ok(Self::Sc),
            "SL" => Ok(Self::Sl),
            "SG" => Ok(Self::Sg),
            "SX" => Ok(Self::Sx),
            "SK" => Ok(Self::Sk),
            "SI" => Ok(Self::Si),
            "SB" => Ok(Self::Sb),
            "SO" => Ok(Self::So),
            "ZA" => Ok(Self::Za),
            "GS" => Ok(Self::Gs),
            "SS" => Ok(Self::Ss),
            "ES" => Ok(Self::Es),
            "LK" => Ok(Self::Lk),
            "SD" => Ok(Self::Sd),
            "SR" => Ok(Self::Sr),
            "SJ" => Ok(Self::Sj),
            "SZ" => Ok(Self::Sz),
            "SE" => Ok(Self::Se),
            "CH" => Ok(Self::Ch),
            "SY" => Ok(Self::Sy),
            "TW" => Ok(Self::Tw),
            "TJ" => Ok(Self::Tj),
            "TZ" => Ok(Self::Tz),
            "TH" => Ok(Self::Th),
            "TL" => Ok(Self::Tl),
            "TG" => Ok(Self::Tg),
            "TK" => Ok(Self::Tk),
            "TO" => Ok(Self::To),
            "TT" => Ok(Self::Tt),
            "TN" => Ok(Self::Tn),
            "TR" => Ok(Self::Tr),
            "TM" => Ok(Self::Tm),
            "TC" => Ok(Self::Tc),
            "TV" => Ok(Self::Tv),
            "UG" => Ok(Self::Ug),
            "UA" => Ok(Self::Ua),
            "AE" => Ok(Self::Ae),
            "GB" => Ok(Self::Gb),
            "US" => Ok(Self::Us),
            "UM" => Ok(Self::Um),
            "UY" => Ok(Self::Uy),
            "UZ" => Ok(Self::Uz),
            "VU" => Ok(Self::Vu),
            "VE" => Ok(Self::Ve),
            "VN" => Ok(Self::Vn),
            "VG" => Ok(Self::Vg),
            "VI" => Ok(Self::Vi),
            "WF" => Ok(Self::Wf),
            "EH" => Ok(Self::Eh),
            "YE" => Ok(Self::Ye),
            "ZM" => Ok(Self::Zm),
            "ZW" => Ok(Self::Zw),
            other => Err(crate::error::UnknownVariant::new(other)),
        }
    }
    /// Returns `true` if this value is the forward-compatibility
    /// [`Landescode::Unknown`] catch-all (an out-of-schema value).
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
    /// Returns `true` if this value is a known, schema-defined variant.
    pub const fn is_known(&self) -> bool {
        !self.is_unknown()
    }
}
impl std::fmt::Display for Landescode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_wire())
    }
}
impl AsRef<str> for Landescode {
    fn as_ref(&self) -> &str {
        self.as_wire()
    }
}
#[cfg(feature = "versioned")]
impl crate::bo4e_enum_sealed::Sealed for Landescode {}
#[cfg(feature = "versioned")]
impl crate::Bo4eEnum for Landescode {
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
impl crate::Bo4eStrict for Landescode {
    fn collect_unknown_enums(&self, path: &str, out: &mut Vec<String>) {
        if self.is_unknown() {
            out.push(path.to_owned());
        }
    }
}
#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for Landescode {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <String as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}
/// Encodes as the canonical BO4E wire string, borrowed from `as_wire` — no
/// intermediate `String` or `serde_json::Value` is allocated.
#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Landescode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let s: &str = self.as_wire();
        <&str as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&s, buf)
    }
}
/// Decodes leniently, matching the `serde` path: a value the schema does not
/// define becomes [`Landescode::Unknown`] rather than a decode error, so a
/// database row written by a newer schema version still reads back.
///
/// Use [`Landescode::from_wire`] on a `String` column, or check
/// [`Landescode::is_known`], where out-of-schema values must be rejected.
#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for Landescode {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let s = <&str as sqlx::Decode<sqlx::Postgres>>::decode(value)?;
        Ok(Self::from_wire(s).unwrap_or(Self::Unknown))
    }
}
#[cfg(test)]
impl proptest::arbitrary::Arbitrary for Landescode {
    type Parameters = ();
    type Strategy = proptest::strategy::BoxedStrategy<Self>;
    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        use proptest::prelude::*;
        proptest::sample::select(Self::VARIANTS.to_vec()).boxed()
    }
}
