use crate::error::config::ConfigError;
use crate::error::encryption::EncryptionError;
use crate::error::foundation::FoundationError;
use crate::error::io::IoError;
use crate::error::keyring::KeyringError;
use crate::error::structured_file::StructuredFileError;
use crate::error::wallet_config::WalletConfigError;

use ic_agent::identity::PemError;
use ic_identity_hsm::HardwareIdentityError;

use std::path::PathBuf;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdentityError {
    #[error("Cannot delete the default identity.")]
    CannotDeleteDefaultIdentity(),

    #[error("Cannot delete the anonymous identity.")]
    CannotDeleteAnonymousIdentity(),

    #[error("Cannot create an anonymous identity.")]
    CannotCreateAnonymousIdentity(),

    #[error("Failed to clean up previous creation attempts: {0}")]
    CleanupPreviousCreationAttemptsFailed(IoError),

    #[error("Convert secret key to sec1 Pem failed: {0}")]
    ConvertSecretKeyToSec1PemFailed(Box<sec1::Error>),

    #[error("Cannot create identity directory: {0}")]
    CreateIdentityDirectoryFailed(IoError),

    #[error("Failed to create mnemonic from phrase: {0}")]
    CreateMnemonicFromPhraseFailed(String),

    #[error("Failed to create temporary identity directory: {0}")]
    CreateTemporaryIdentityDirectoryFailed(IoError),

    #[error("Cannot save PEM content for an HSM.")]
    CannotSavePemContentForHsm(),

    #[error("Failed to decrypt PEM file: {0}")]
    DecryptPemFileFailed(PathBuf, EncryptionError),

    #[error("Failed to derive extended secret key from path: {0}")]
    DeriveExtendedKeyFromPathFailed(bip32::Error),

    #[error("Failed to display linked wallets: {0}")]
    DisplayLinkedWalletsFailed(WalletConfigError),

    #[error("If you want to remove an identity with configured wallets, please use the --drop-wallets flag.")]
    DropWalletsFlagRequiredToRemoveIdentityWithWallets(),

    #[error("Cannot encrypt PEM file: {0}")]
    EncryptPemFileFailed(PathBuf, EncryptionError),

    #[error("Failed to ensure identity configuration directory exists: {0}")]
    EnsureIdentityConfigurationDirExistsFailed(IoError),

    #[error("Failed to generate a fresh encryption configuration: {0}")]
    GenerateFreshEncryptionConfigurationFailed(EncryptionError),

    #[error("Failed to generate a fresh secp256k1 key: {0}")]
    GenerateFreshSecp256k1KeyFailed(Box<sec1::Error>),

    #[error("Failed to get config directory for identity manager: {0}")]
    GetConfigDirectoryFailed(ConfigError),

    #[error("Failed to get shared network data directory: {0}")]
    GetSharedNetworkDataDirectoryFailed(ConfigError),

    #[error("Failed to get principal of identity: {0}")]
    GetIdentityPrincipalFailed(String),

    #[error("Failed to get legacy pem path: {0}")]
    GetLegacyPemPathFailed(FoundationError),

    #[error("Identity already exists.")]
    IdentityAlreadyExists(),

    #[error("Identity {0} does not exist at '{1}'.")]
    IdentityDoesNotExist(String, PathBuf),

    #[error("Failed to instantiate hardware identity for identity '{0}': {1}.")]
    InstantiateHardwareIdentityFailed(String, Box<HardwareIdentityError>),

    #[error("Failed to load configuration for identity '{0}': {1}")]
    LoadIdentityConfigurationFailed(String, StructuredFileError),

    #[error("Failed to load identity manager configuration: {0}")]
    LoadIdentityManagerConfigurationFailed(StructuredFileError),

    #[error("Failed to load PEM file from keyring for identity '{0}': {1}")]
    LoadPemFromKeyringFailed(Box<String>, KeyringError),

    #[error("Failed to migrate legacy identity")]
    MigrateLegacyIdentityFailed(IoError),

    #[error("Cannot read identity file '{0}': {1:#}")]
    ReadIdentityFileFailed(String, Box<PemError>),

    #[error("Failed to read pem file: {0}")]
    ReadPemFileFailed(IoError),

    #[error("Failed to remove identity directory: {0}")]
    RemoveIdentityDirectoryFailed(IoError),

    #[error("Failed to remove identity from keyring: {0}")]
    RemoveIdentityFromKeyringFailed(KeyringError),

    #[error("Failed to remove identity file: {0}")]
    RemoveIdentityFileFailed(IoError),

    #[error("Cannot rename identity directory: {0}")]
    RenameIdentityDirectoryFailed(IoError),

    #[error("Failed to rename temporary directory to permanent identity directory: {0}")]
    RenameTemporaryIdentityDirectoryFailed(IoError),

    #[error("Failed to rename '{0}' to '{1}' in the global wallet config: {2}")]
    RenameWalletFailed(Box<String>, Box<String>, WalletConfigError),

    #[error("An Identity named {0} cannot be created as it is reserved for internal use.")]
    ReservedIdentityName(String),

    #[error("Failed to save identity configuration: {0}")]
    SaveIdentityConfigurationFailed(StructuredFileError),

    #[error("Failed to save identity manager configuration: {0}")]
    SaveIdentityManagerConfigurationFailed(StructuredFileError),

    #[error("Failed to switch back over to the identity you're replacing: {0}")]
    SwitchBackToIdentityFailed(Box<IdentityError>),

    #[error("Failed to switch over default identity settings: {0}")]
    SwitchDefaultIdentitySettingsFailed(Box<IdentityError>),

    #[error("Failed to temporarily switch over to anonymous identity: {0}")]
    SwitchToAnonymousIdentityFailed(Box<IdentityError>),

    #[error("Could not translate pem file to text: {0}")]
    TranslatePemContentToTextFailed(FromUtf8Error),

    #[error(
        "Ed25519 v1 keys (those generated by OpenSSL) are not supported. Try again with a v2 key"
    )]
    UnsupportedKeyVersion(),

    #[error("Failed to validate PEM content: {0}")]
    ValidatePemContentFailed(Box<PemError>),

    #[error("Cannot write PEM file: {0}")]
    WritePemFileFailed(IoError),

    #[error("Failed to write PEM to keyring: {0}")]
    WritePemToKeyringFailed(KeyringError),
}
