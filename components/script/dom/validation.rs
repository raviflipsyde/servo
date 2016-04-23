/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub trait Validatable {
	fn get_attribute_value(&self, input_attr_name:&String) -> Option<&str>;
	//fn get_value_for_validation()
}
