mod contact;
mod facebook;
mod github;
mod instagram;

pub(crate) use contact::Contact;
pub(crate) use facebook::Facebook;
pub(crate) use github::Github;
pub(crate) use instagram::Instagram;

pub(crate) enum UserData {
    Contact(Contact),
    Facebook(Facebook),
    Github(Github),
    Instagram(Instagram),
}
