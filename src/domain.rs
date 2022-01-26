use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
// SubscriberName is a turple struct - a new type,
// with a single (unnamed) field of type String.

/* SubscriberName is a proper new type, not just an alias -
   it does not inherit any of the methods available on String
   and trying to assign a String to a variable of type
   SubscriberName will trigger a compiler error

   /// let name: SubscriberName = "A string".to_string();
*/

/* NOTE: The inner field of SubscriberName is private to this module,
it can only be accessed from code within our domain module. */

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    /// Returns an instance of `SubscriberName` if the input satisfies all
    /// our validation constraints on subscriber names, otherwise panic!

    pub fn parse(name: String) -> Result<SubscriberName, String> {
        // `.trim()` returns a view over the input `name` without trailing whitespace-like characters.
        // `.is_empty` checks if the view contains any character.
        let is_empty_or_whitespace = name.trim().is_empty();

        // A grapheme is defined by the Unicode standard as a "user-perceived"
        // character: `å` is a single grapheme, but it is composed of two characters
        // (`a` and ``).
        //
        // `graphemes` returns an iterator over the graphemes in the input `name`.
        // `true` specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = name.graphemes(true).count() > 256;

        // Iterate over all characters in the input `name` to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = name
            .chars()
            .any(|item| forbidden_characters.contains(&item));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", name))
        } else {
            Ok(Self(name))
        }
    }
}

// The caller gets a shared reference to the inner string.
// This gives the caller **read-only** access,
// they have no way to compromise our invariants!
impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
