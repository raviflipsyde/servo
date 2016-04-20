/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::ValidityStateBinding;
use dom::bindings::codegen::Bindings::ValidityStateBinding::ValidityStateMethods;
use dom::bindings::global::GlobalRef;
use dom::bindings::js::{JS, Root};
use dom::bindings::reflector::{Reflector, reflect_dom_object};
use dom::element::Element;
use dom::window::Window;
use dom::node::Node;
use dom::htmlelement::HTMLElement;
use dom::htmlinputelement::HTMLInputElement;
use dom::htmlselectelement::HTMLSelectElement;
use dom::htmlbuttonelement::HTMLButtonElement;
use dom::htmlobjectelement::HTMLObjectElement;
use dom::htmltextareaelement::HTMLTextAreaElement;
use dom::validation::Validatable;
use dom::bindings::inheritance::{Castable, ElementTypeId, HTMLElementTypeId, NodeTypeId};
use dom::htmlformelement::{FormControl, FormDatum, FormSubmitter, HTMLFormElement};
use dom::htmloptionelement::HTMLOptionElement;
use dom::bindings::codegen::Bindings::HTMLSelectElementBinding::HTMLSelectElementMethods;
use dom::bindings::codegen::Bindings::HTMLOptionElementBinding::HTMLOptionElementMethods;
use dom::bindings::codegen::Bindings::HTMLTextAreaElementBinding::HTMLTextAreaElementMethods;
//use dom::htmlformelement::FormDatumGetter;
// https://html.spec.whatwg.org/multipage/#validity-states
#[derive_JSTraceable]
#[derive_HeapSizeOf]
pub enum ValidityStatus {
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

// https://html.spec.whatwg.org/multipage/#validitystate
#[dom_struct]
pub struct ValidityState {
    reflector_: Reflector,
    element: JS<Element>,
    state: ValidityStatus
}


impl ValidityState {
    fn new_inherited(element: &Element) -> ValidityState {
        ValidityState {
            reflector_: Reflector::new(),
            element: JS::from_ref(element),
            state: ValidityStatus::Valid
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
                
                let element1 = self.element.downcast::<HTMLInputElement>().unwrap();
                let data = element1.form_datum(Some(FormSubmitter::InputElement(element1)));

                match data {
                    Some(data_object) => {                        
                        for attr in self.element.attrs().iter() {
                            let attr_name = &**attr.name();
                            if str::eq(attr_name,"required") {
                                if data_object.value.is_empty() {
                                    println!("Error - No input has been provided for a required field");
                                    return false;
                                } else {
                                    println!("Value is {}", data_object.value);
                                    return true;
                                }
                            }                    
                        }
                    },
                    None => {
                        println!("None");
                        return true;
                    }
                }                
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLButtonElement)) => {
               return true;
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLObjectElement)) => {
                return true;
               
            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLSelectElement)) => {
                let element1 = self.element.downcast::<HTMLSelectElement>().unwrap();
                let node = element1.upcast::<Node>();
                /*if element1.Name().is_empty() {
                    return false;
                }*/
                
                for attr in self.element.attrs().iter() {
                    let attr_name = &**attr.name();
                    if str::eq(attr_name,"required") {
                        for opt in node.traverse_preorder().filter_map(Root::downcast::<HTMLOptionElement>) {
                            let element = opt.upcast::<Element>();
                            if opt.Selected() && element.enabled_state() && !opt.Value().is_empty(){
                                return true;
                            }
                        }
                    }
                }                
                return false;

            },
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLTextAreaElement)) => {
                let textarea = self.element.downcast::<HTMLTextAreaElement>().unwrap();
                let name = textarea.Name();
                if !name.is_empty() {
                    println!("Value in text area is {}", textarea.Value())
                }
                
            },
            NodeTypeId::Element(_)  => {
                println!("6");
            }
            NodeTypeId::CharacterData(_)  => {
                println!("7");
            }
            NodeTypeId::Document(_)  => {
                println!("8");
            }
            NodeTypeId::DocumentFragment  => {
                println!("9");
            }
            NodeTypeId::DocumentType  => {
                println!("10");
            }

        };
        
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-typemismatch
    fn TypeMismatch(&self) -> bool {
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

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-patternmismatch
    fn PatternMismatch(&self) -> bool {
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

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-toolong
    fn TooLong(&self) -> bool {
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

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-tooshort
    fn TooShort(&self) -> bool {
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

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-rangeunderflow
    fn RangeUnderflow(&self) -> bool {
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

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-rangeoverflow
    fn RangeOverflow(&self) -> bool {
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

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-stepmismatch
    fn StepMismatch(&self) -> bool {
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
        return self.ValueMissing();
    }
}