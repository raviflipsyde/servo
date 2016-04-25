/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use dom::bindings::codegen::Bindings::AttrBinding::AttrMethods;
use dom::bindings::codegen::Bindings::HTMLOptionElementBinding::HTMLOptionElementMethods;
use dom::bindings::codegen::Bindings::HTMLSelectElementBinding::HTMLSelectElementMethods;
use dom::bindings::codegen::Bindings::HTMLTextAreaElementBinding::HTMLTextAreaElementMethods;
use dom::bindings::codegen::Bindings::ValidityStateBinding;
use dom::bindings::codegen::Bindings::ValidityStateBinding::ValidityStateMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::inheritance::{Castable, ElementTypeId, HTMLElementTypeId, NodeTypeId};
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::element::Element;
use dom::htmlbuttonelement::HTMLButtonElement;
use dom::htmlelement::HTMLElement;
use dom::htmlformelement::{FormControl, FormDatum, FormSubmitter, HTMLFormElement};
use dom::htmlinputelement::HTMLInputElement;
use dom::htmlobjectelement::HTMLObjectElement;
use dom::htmloptionelement::HTMLOptionElement;
use dom::htmlselectelement::HTMLSelectElement;
use dom::htmltextareaelement::HTMLTextAreaElement;
use dom::node::Node;
use dom::validation::Validatable;
use dom::window::Window;
use regex::Regex;
use util::str::DOMString;

/// https://html.spec.whatwg.org/multipage/#validity-states
#[derive(JSTraceable, HeapSizeOf)]
pub enum ValidityStates {
    ValueMissing,
    TypeMismatch,
    PatternMismatch,
    TooLong,
    TooShort,
    RangeUnderflow,
    RangeOverflow,
    StepMismatch,
    BadInput,
    CustomError,
    Valid
}

/// https://html.spec.whatwg.org/multipage/#validitystate
#[dom_struct]
pub struct ValidityState {
    reflector_: Reflector,
    element: JS<Element>,
    state: ValidityStates
}


impl ValidityState {
    fn new_inherited(element: &Element) -> ValidityState {
        ValidityState {
            reflector_: Reflector::new(),
            element: JS::from_ref(element),
            state: ValidityStates::Valid
        }
    }
    pub fn new(window: &Window, element: &Element) -> Root<ValidityState> {
        reflect_dom_object(box ValidityState::new_inherited(element),
                           GlobalRef::Window(window),
                           ValidityStateBinding::Wrap)
    }
}


impl ValidityStateMethods for ValidityState {

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-valuemissing
    fn ValueMissing(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("required"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                return false;
                            },
                            None => {
                                println!("Error - Value missing in html input element");
                                return true;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
                //let data = element1.form_datum(Some(FormSubmitter::InputElement(element1)));
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
               return false;
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                return false;
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                /*let attr_value_check = self.element.get_attribute_by_name(DOMString::from("required"))
                .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let html_select_element = self.element.downcast::<HTMLSelectElement>().unwrap();
                        let input_value_check = html_select_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                return false;
                            },
                            None => {
                                println!("Error - Value missing in html select area element");
                                return true;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }*/
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("required"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let html_textarea_element = self.element.downcast::<HTMLTextAreaElement>().unwrap();
                        let input_value_check = html_textarea_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                return false;
                            },
                            None => {
                                println!("Error - Value missing in html text area element");
                                return true;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(_)  => {
                return false;
            }
            NodeTypeId::CharacterData(_)  => {
                return false;
            }
            NodeTypeId::Document(_)  => {
                return false;
            }
            NodeTypeId::DocumentFragment  => {
                return false;
            }
            NodeTypeId::DocumentType  => {
                return false;
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-typemismatch
    fn TypeMismatch(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let regex_email: Regex = Regex::new(r"/^[a-zA-Z0-9.!#$%&'*+\/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-] \
                    {0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$/").unwrap();
                let regex_url: Regex = Regex::new(r"").unwrap();
                let regex_number: Regex = Regex::new(r"").unwrap();
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("type"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                if attr_value == "email"{
                                    return !regex_email.is_match(&*input_value);
                                }
                                else if attr_value == "url" {
                                    return !regex_url.is_match(&*input_value);
                                }
                                else if attr_value == "number" {
                                    return !regex_number.is_match(&*input_value);
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("Error - Value missing in html input element");
                                return true;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
                //let data = element1.form_datum(Some(FormSubmitter::InputElement(element1)));
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-patternmismatch
    fn PatternMismatch(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("pattern"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let regex = Regex::new(&*attr_value).unwrap();
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                return !regex.is_match(&*input_value);
                            },
                            None => {
                                println!("No Error - No Value in html input element");
                                return false;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-toolong
    fn TooLong(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("maxlength"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let maxlength = attr_value.parse().unwrap();
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                if input_value.len() > maxlength {
                                    return true;
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("No Error - No Value in html input element");
                                return false;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("maxlength"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let maxlength = attr_value.parse().unwrap();
                        let html_textarea_element = self.element.downcast::<HTMLTextAreaElement>().unwrap();
                        let input_value_check = html_textarea_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                             if input_value.len() > maxlength {
                                    return true;
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("Error - Value missing in html text area element");
                                return true;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-tooshort
    fn TooShort(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("minlength"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let minlength = attr_value.parse().unwrap();
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                if input_value.len() < minlength {
                                    return true;
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("No Error - No Value in html input element");
                                return false;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("minlength"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let minlength = attr_value.parse().unwrap();
                        let html_textarea_element = self.element.downcast::<HTMLTextAreaElement>().unwrap();
                        let input_value_check = html_textarea_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                             if input_value.len() < minlength {
                                    return true;
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("Error - Value missing in html text area element");
                                return true;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-rangeunderflow
    fn RangeUnderflow(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("min"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let min: f32 = attr_value.parse().unwrap();
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                let text_value: f32 = input_value.parse().unwrap();
                                if text_value < min {
                                    return true;
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("No Error - No Value in html input element");
                                return false;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-rangeoverflow
    fn RangeOverflow(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("max"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let max: f32 = attr_value.parse().unwrap();
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                let text_value: f32 = input_value.parse().unwrap();
                                if text_value > max {
                                    return true;
                                }
                                else {
                                    return false;
                                }
                            },
                            None => {
                                println!("No Error - No Value in html input element");
                                return false;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-stepmismatch
    fn StepMismatch(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                let attr_value_check = self.element.get_attribute_by_name(DOMString::from("step"))
                    .map(|s| s.Value());
                match attr_value_check {
                    Some(attr_value) => {
                        let step: f32 = attr_value.parse().unwrap();
                        let html_input_element = self.element.downcast::<HTMLInputElement>().unwrap();
                        let input_value_check = html_input_element.get_value_for_validation();
                        match input_value_check {
                            Some(input_value) => {
                                let text_value: f32 = input_value.parse().unwrap();
                                if text_value % step == 0.0_f32 {
                                    return false;
                                }
                                else {
                                    return true;
                                }
                            },
                            None => {
                                println!("No Error - No Value in html input element");
                                return false;
                            }
                        }
                    },
                    None => {
                        return false;
                    }
                }
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-badinput
    fn BadInput(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                println!("1");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-customerror
    fn CustomError(&self) -> bool {
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                println!("1");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
                //let element = self.downcast::<HTMLButtonElement>().unwrap();
                println!("2");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                //let element = self.downcast::<HTMLObjectElement>().unwrap();
                println!("3");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                //let element = self.downcast::<HTMLSelectElement>().unwrap();
               println!("4");
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                //let element = self.downcast::<HTMLTextAreaElement>().unwrap();
               println!("5");
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("6");
            }
            NodeTypeId::Document(_)  => {
                println!("6");
            }
            NodeTypeId::DocumentFragment  => {
                println!("6");
            }
            NodeTypeId::DocumentType  => {
                println!("6");
            }
        };
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-valid
    fn Valid(&self) -> bool {
        return !(self.ValueMissing()|
            self.TypeMismatch()|
            self.PatternMismatch()|
            self.TooLong()|
            self.TooShort()|
            self.RangeUnderflow()|
            self.RangeOverflow()|
            self.StepMismatch()|
            self.BadInput()|
            self.CustomError());
    }
}
