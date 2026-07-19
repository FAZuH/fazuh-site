use dioxus::prelude::*;
use fazuh_utils::contact::ContactForm;
use fazuh_utils::contact::ContactResponse;
use fazuh_utils::contact::submit_contact;
use fazuh_utils::validation;

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut response = use_signal(|| None::<ContactResponse>);
    let mut is_pending = use_signal(|| false);

    let on_submit = move |evt: FormEvent| {
        evt.prevent_default();
        is_pending.set(true);
        response.set(None);

        match ContactForm::builder(name())
            .email(email())
            .message(message())
            .build()
        {
            Ok(form) => {
                spawn(async move {
                    match submit_contact(form).await {
                        Ok(resp) => {
                            is_pending.set(false);
                            if resp.success {
                                name.set(String::new());
                                email.set(String::new());
                                message.set(String::new());
                            }
                            response.set(Some(resp));
                        }
                        Err(err) => {
                            is_pending.set(false);
                            let error_msg = match &err {
                                ServerFnError::Request(e) => format!("Network error: {e}"),
                                _ => format!("Server error: {err}"),
                            };
                            response.set(Some(ContactResponse {
                                success: false,
                                message: error_msg,
                                errors: None,
                            }));
                        }
                    }
                });
            }
            Err(errors) => {
                is_pending.set(false);
                response.set(Some(ContactResponse {
                    success: false,
                    message: "Please fix the form errors and try again.".to_string(),
                    errors: Some(validation::format_errors(&errors)),
                }));
            }
        }
    };

    rsx! {
        section {
            id: "contact",
            class: "py-16 md:py-20",
            div {
                class: "max-w-[960px] mx-auto px-6",
                h2 {
                    class: "text-xl font-bold text-ink mb-2",
                    "Contact"
                }
                p {
                    class: "text-sm text-mute mb-8 max-w-[65ch]",
                    "Send me a message."
                }

                div {
                    class: "border-t border-hairline pt-6 max-w-[640px]",
                    {response.read().as_ref().map(|resp| {
                        let alert_class = if resp.success {
                            "bg-success/10 text-success border border-success/30"
                        } else {
                            "bg-danger/10 text-danger border border-danger/30"
                        };
                        rsx! {
                            div {
                                class: "p-3 rounded-sm mb-6 text-sm {alert_class}",
                                "{resp.message}"
                            }
                        }
                    })}

                    form {
                        onsubmit: on_submit,
                        novalidate: true,
                        class: "flex flex-col gap-5",
                        FieldInput {
                            label: "Name",
                            input_type: "text",
                            placeholder: "Your name",
                            value: name(),
                            oninput: move |evt: FormEvent| name.set(evt.value()),
                            errors: response.read().as_ref().and_then(|r| r.errors.as_ref()).and_then(|e| e.get("name").cloned()).unwrap_or_default(),
                        }
                        FieldInput {
                            label: "Email",
                            input_type: "email",
                            placeholder: "you@example.com",
                            value: email(),
                            oninput: move |evt: FormEvent| email.set(evt.value()),
                            errors: response.read().as_ref().and_then(|r| r.errors.as_ref()).and_then(|e| e.get("email").cloned()).unwrap_or_default(),
                        }
                        FieldTextarea {
                            label: "Message",
                            placeholder: "Your message",
                            value: message(),
                            oninput: move |evt: FormEvent| message.set(evt.value()),
                            errors: response.read().as_ref().and_then(|r| r.errors.as_ref()).and_then(|e| e.get("message").cloned()).unwrap_or_default(),
                        }
                        button {
                            class: "w-full sm:w-auto px-5 py-2 text-sm font-medium bg-ink text-canvas rounded-sm \
                                hover:bg-ink-deep transition-colors disabled:opacity-60 \
                                disabled:cursor-not-allowed disabled:hover:bg-ink flex items-center gap-2 justify-center",
                            r#type: "submit",
                            disabled: is_pending(),
                            if is_pending() {
                                svg {
                                    class: "animate-spin h-4 w-4",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    circle {
                                        class: "opacity-25",
                                        cx: "12",
                                        cy: "12",
                                        r: "10",
                                        stroke: "currentColor",
                                        stroke_width: "4",
                                    }
                                    path {
                                        class: "opacity-75",
                                        fill: "currentColor",
                                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z",
                                    }
                                }
                                "Sending..."
                            } else {
                                "[>] Send"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FieldInput(
    label: &'static str,
    input_type: &'static str,
    placeholder: &'static str,
    value: String,
    oninput: EventHandler<FormEvent>,
    errors: Vec<String>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2",
            label {
                class: "text-xs font-medium text-mute",
                "{label}"
            }
            input {
                class: "px-3 py-2.5 bg-surface-soft border border-hairline rounded-sm text-ink \
                    placeholder:text-ash text-sm focus:outline-none focus:bg-canvas \
                    focus:border-ink transition-colors",
                r#type: input_type,
                name: label,
                placeholder: placeholder,
                required: true,
                value,
                oninput,
            }
            {errors.first().map(|e| rsx! {
                p { class: "text-xs text-red-600 mt-1", "{e}" }
            })}
        }
    }
}

#[component]
fn FieldTextarea(
    label: &'static str,
    placeholder: &'static str,
    value: String,
    oninput: EventHandler<FormEvent>,
    errors: Vec<String>,
) -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-2",
            label {
                class: "text-xs font-medium text-mute",
                "{label}"
            }
            textarea {
                class: "px-3 py-3 bg-surface-soft border border-hairline rounded-sm text-ink \
                    placeholder:text-ash text-sm focus:outline-none focus:bg-canvas \
                    focus:border-ink transition-colors resize-y min-h-[140px]",
                name: label,
                placeholder: placeholder,
                required: true,
                value,
                oninput,
            }
            {errors.first().map(|e| rsx! {
                p { class: "text-xs text-danger mt-1", "{e}" }
            })}
        }
    }
}
