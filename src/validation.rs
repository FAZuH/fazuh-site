#[cfg(feature = "server")]
use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use validator::Validate;
#[cfg(feature = "server")]
use validator::ValidationErrors;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct ContactForm {
    #[validate(length(
        min = 2,
        max = 100,
        message = "Name must be between 2 and 100 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,

    #[validate(length(
        min = 10,
        max = 1000,
        message = "Message must be between 10 and 1000 characters"
    ))]
    pub message: String,
}

#[cfg(feature = "server")]
pub fn format_errors(errors: &ValidationErrors) -> HashMap<String, Vec<String>> {
    let mut formatted = HashMap::new();

    for (field, error_kind) in errors.field_errors() {
        let messages: Vec<String> = error_kind
            .iter()
            .map(|e| {
                e.message
                    .clone()
                    .map(|m| m.into_owned())
                    .unwrap_or_else(|| "Invalid field".to_string())
            })
            .collect();

        formatted.insert(field.to_string(), messages);
    }

    formatted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_form() -> ContactForm {
        ContactForm {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            message: "I would like to discuss a project.".to_string(),
        }
    }

    #[test]
    fn valid_form_passes_validation() {
        let form = valid_form();
        let result = form.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_name_shorter_than_2_characters() {
        let form = ContactForm {
            name: "J".to_string(),
            ..valid_form()
        };
        let result = form.validate();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_name_longer_than_100_characters() {
        let form = ContactForm {
            name: "a".repeat(101),
            ..valid_form()
        };
        let result = form.validate();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_message_shorter_than_10_characters() {
        let form = ContactForm {
            message: "ab".to_string(),
            ..valid_form()
        };
        let result = form.validate();
        assert!(result.is_err());
    }

    #[test]
    fn rejects_message_longer_than_1000_characters() {
        let form = ContactForm {
            message: "a".repeat(1001),
            ..valid_form()
        };
        let result = form.validate();
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "server")]
    fn formats_validated_errors_into_field_map() {
        let form = ContactForm {
            name: "J".to_string(),
            email: "bad".to_string(),
            ..valid_form()
        };
        let errors = form.validate().unwrap_err();
        let formatted = format_errors(&errors);

        assert!(
            formatted.contains_key("name"),
            "should include failing field"
        );
        assert!(
            formatted.contains_key("email"),
            "should include another failing field"
        );
        assert!(
            !formatted.contains_key("message"),
            "must not include passing field"
        );
    }

    #[test]
    #[cfg(feature = "server")]
    fn formats_validated_errors_with_correct_messages() {
        let form = ContactForm {
            name: "J".to_string(),
            email: "bad".to_string(),
            ..valid_form()
        };
        let errors = form.validate().unwrap_err();
        let formatted = format_errors(&errors);

        assert_eq!(
            formatted.get("name").unwrap()[0],
            "Name must be between 2 and 100 characters"
        );
        assert_eq!(
            formatted.get("email").unwrap()[0],
            "Please enter a valid email address"
        );
    }
}
