use std::convert::TryFrom;
use std::os::raw::c_ulong;

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};

#[repr(u64)]
#[derive(Debug, FromPrimitive, ToPrimitive)]
pub enum AttributeType {
    Class = 0x0000,
    Token = 0x0001,
    Private = 0x0002,
    Label = 0x0003,
    Application = 0x0010,
    Value = 0x0011,
    ObjectId = 0x0012,
    CertificateType = 0x0080,
    Issuer = 0x0081,
    SerialNumber = 0x0082,
    AcIssuer = 0x0083,
    Owner = 0x0084,
    AttrTypes = 0x0085,
    Trusted = 0x0086,
    CertificateCategory = 0x0087,
    JavaMidpSecurityDomain = 0x0088,
    Url = 0x0089,
    HashOfSubjectPublicKey = 0x008a,
    HashOfIssuerPublicKey = 0x008b,
    NameHashAlgorithm = 0x008c,
    CheckValue = 0x0090,
    KeyType = 0x0100,
    Subject = 0x0101,
    Id = 0x0102,
    Sensitive = 0x0103,
    Encrypt = 0x0104,
    Decrypt = 0x0105,
    Wrap = 0x0106,
    Unwrap = 0x0107,
    Sign = 0x0108,
    SignRecover = 0x0109,
    Verify = 0x010a,
    VerifyRecover = 0x010b,
    Derive = 0x010c,
    StartDate = 0x0110,
    EndDate = 0x0111,
    Modulus = 0x0120,
    ModulusBits = 0x0121,
    PublicExponent = 0x0122,
    PrivateExponent = 0x0123,
    Prime1 = 0x0124,
    Prime2 = 0x0125,
    Exponent1 = 0x0126,
    Exponent2 = 0x0127,
    Coefficient = 0x0128,
    PublicKeyInfo = 0x0129,
    Prime = 0x0130,
    Subprime = 0x0131,
    Base = 0x0132,
    PrimeBits = 0x0133,
    SubPrimeBits = 0x0134,
    ValueBits = 0x0160,
    ValueLen = 0x0161,
    Extractable = 0x0162,
    Local = 0x0163,
    NeverExtractable = 0x0164,
    AlwaysSensitive = 0x0165,
    KeyGenMechanism = 0x0166,
    Modifiable = 0x0170,
    Copyable = 0x0171,
    Destroyable = 0x0172,
    EcParams = 0x0180,
    EcPoint = 0x0181,
    SecondaryAuth = 0x0200,
    AuthPinFlags = 0x0201,
    AlwaysAuthenticate = 0x0202,
    WrapWithTrusted = 0x0210,
    WrapTemplate = 0x40000211,
    UnwrapTemplate = 0x40000212,
    DeriveTemplate = 0x40000213,
    OtpFormat = 0x0220,
    OtpLength = 0x0221,
    OtpTimeInterval = 0x0222,
    OtpUserFriendlyMode = 0x0223,
    OtpChallengeRequirement = 0x0224,
    OtpTimeRequirement = 0x0225,
    OtpCounterRequirement = 0x0226,
    OtpPinRequirement = 0x0227,
    OtpCounter = 0x022e,
    OtpTime = 0x022f,
    OtpUserIdentifier = 0x022a,
    OtpServiceIdentifier = 0x022b,
    OtpServiceLogo = 0x022c,
    OtpServiceLogoType = 0x022d,
    Gostr3410Params = 0x0250,
    Gostr3411Params = 0x0251,
    Gost28147Params = 0x0252,
    HwFeatureType = 0x0300,
    ResetOnInit = 0x0301,
    HasReset = 0x0302,
    PixelX = 0x0400,
    PixelY = 0x0401,
    Resolution = 0x0402,
    CharRows = 0x0403,
    CharColumns = 0x0404,
    Color = 0x0405,
    BitsPerPixel = 0x0406,
    CharSets = 0x0480,
    EncodingMethods = 0x0481,
    MimeTypes = 0x0482,
    MechanismType = 0x0500,
    RequiredCmsAttributes = 0x0501,
    DefaultCmsAttributes = 0x0502,
    SupportedCmsAttributes = 0x0503,
    AllowedMechanisms = 0x40000600,
    VendorDefined = 0x80000000,
}

#[cfg(target_pointer_width = "32")]
impl TryFrom<c_ulong> for AttributeType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        AttributeType::from_u32(value).ok_or(())
    }
}
#[cfg(target_pointer_width = "32")]
impl TryFrom<AttributeType> for c_ulong {
    type Error = ();
    fn try_from(value: AttributeType) -> Result<Self, Self::Error> {
        AttributeType::to_u32(&value).ok_or(())
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<c_ulong> for AttributeType {
    type Error = ();
    fn try_from(value: c_ulong) -> Result<Self, Self::Error> {
        AttributeType::from_u64(value).ok_or(())
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<AttributeType> for c_ulong {
    type Error = ();
    fn try_from(value: AttributeType) -> Result<Self, Self::Error> {
        AttributeType::to_u64(&value).ok_or(())
    }
}
