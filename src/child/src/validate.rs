use ic_scalable_misc::{
    enums::{api_error_type::ApiError, validation_type::ValidationType},
    helpers::validation_helper::Validator,
    models::validation_models::ValidateField,
};

use shared::report_model::PostReport;

pub fn validate_post_report(post_report: PostReport) -> Result<(), ApiError> {
    let validator_fields = vec![
        ValidateField(
            ValidationType::StringLength(post_report.message, 0, 500),
            "message".to_string(),
        ),
    ];

    Validator(validator_fields).validate()
}
