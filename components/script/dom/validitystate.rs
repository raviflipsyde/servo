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
        for attr in self.element.attrs().iter() {
                    let n = &**attr.name();
                    let v = &**attr.value();
                    if str::eq(n,"required") {
                        println!("attrVal = {}, {}",n,v );
                    }                    
                }
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-typemismatch
    fn TypeMismatch(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-patternmismatch
    fn PatternMismatch(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-toolong
    fn TooLong(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-tooshort
    fn TooShort(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-rangeunderflow
    fn RangeUnderflow(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-rangeoverflow
    fn RangeOverflow(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-stepmismatch
    fn StepMismatch(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-badinput
    fn BadInput(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-customerror
    fn CustomError(&self) -> bool {
        false
    }

    // https://html.spec.whatwg.org/multipage/#dom-validitystate-valid
    fn Valid(&self) -> bool {
        
        let element = match self.element.upcast::<Node>().type_id() {
            NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLInputElement)) => {
                self.ValueMissing();
                let element1 = self.element.downcast::<HTMLInputElement>().unwrap();
                let data = element1.form_datum(Some(FormSubmitter::InputElement(element1)));

                match data {
                    Some(x) => {
                        println!("x is {}", x.value);
                    },
                    None => {
                        println!("None");
                    }
                }              

                //println!("1");
               
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
}
