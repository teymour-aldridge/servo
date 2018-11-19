/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::dom::bindings::codegen::Bindings::HTMLDataListElementBinding;
use crate::dom::bindings::codegen::Bindings::HTMLDataListElementBinding::HTMLDataListElementMethods;
use crate::dom::bindings::inheritance::Castable;
use crate::dom::bindings::root::DomRoot;
use crate::dom::document::Document;
use crate::dom::element::Element;
use crate::dom::htmlcollection::{CollectionFilter, HTMLCollection};
use crate::dom::htmlelement::HTMLElement;
use crate::dom::htmloptionelement::HTMLOptionElement;
use crate::dom::node::{window_from_node, Node};
use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};

#[dom_struct]
pub struct HTMLDataListElement {
    htmlelement: HTMLElement,
}

impl HTMLDataListElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLDataListElement {
        HTMLDataListElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> DomRoot<HTMLDataListElement> {
        Node::reflect_node(
            Box::new(HTMLDataListElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            HTMLDataListElementBinding::Wrap,
        )
    }
}

impl HTMLDataListElementMethods for HTMLDataListElement {
    // https://html.spec.whatwg.org/multipage/#dom-datalist-options
    fn Options(&self) -> DomRoot<HTMLCollection> {
        #[derive(JSTraceable, MallocSizeOf)]
        struct HTMLDataListOptionsFilter;
        impl CollectionFilter for HTMLDataListOptionsFilter {
            fn filter(&self, elem: &Element, _root: &Node) -> bool {
                elem.is::<HTMLOptionElement>()
            }
        }
        let filter = Box::new(HTMLDataListOptionsFilter);
        let window = window_from_node(self);
        HTMLCollection::create(&window, self.upcast(), filter)
    }
}
