//! Pages module - all route pages

pub mod home;
pub mod massload;
pub mod register;
pub mod protect;
pub mod explore;
pub mod how_it_works;

pub use home::HomePage;
pub use massload::MassloadPage;
pub use register::{
    RegisterPage,
    RegisterMusicalWorkPage,
    RegisterRecordingPage,
    RegisterReleasePage,
    RegisterArtistPage,
    RegisterLegalEntityPage,
};
pub use protect::ProtectPage;
pub use explore::ExplorePage;
pub use how_it_works::HowItWorksPage;
