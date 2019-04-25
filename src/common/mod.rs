use std::ops::Index;

pub struct Segment {
    pub name: String,
    pub elements: Vec<Element>,
    pub usage: Usage,
}

impl Index<usize> for Segment {
    type Output = Element;

    fn index(&self, index: usize) -> &Element {
        match index {
            0 => &Element {
                value: self.name,
                element_type: ElementType::SegmentName(),
            },
            i => {
                if i > self.elements.len() {
                    panic!("Out of bounds");
                }
                &self.elements[i]
            }
        }
    }
}

pub struct Element {
    pub element_type: ElementType,
    pub value: String,
}

pub enum Usage {
    Required(),
    Situational(),
    NotUsed(),
}

pub enum ElementType {
    Alphanumeric(),
    Numeric(),
    SegmentName(),
}

pub struct Isa {
    pub authorization_information: AuthorizationInformation,
    pub security_information_qualifier: Element,
    pub security_information: Element,
    pub sender: InterchangeSender,
    pub receiver: InterchangeReceiver,
    pub interchange_date: Element,
    pub interchange_time: Element,
    pub interchange_control_standard_identifier: Element,
    pub interchange_control_version_number: Element,
    pub interchange_control_number: Element,
    pub acknowledgement_requested: Element,
    pub test_indicator: Element,
    pub subelement_separator: Element,
}

impl Index<usize> for Isa {
    type Output = Element;

    fn index(&self, position: usize) -> &Element {
        match position {
            1 => &self.authorization_information.qualifier,
            2 => &self.authorization_information.information,
            3 => &self.security_information_qualifier,
            4 => &self.security_information,
            5 => &self.sender.qualifier,
            6 => &self.sender.id,
            7 => &self.receiver.qualifier,
            8 => &self.receiver.id,
            9 => &self.interchange_date,
            10 => &self.interchange_time,
            11 => &self.interchange_control_standard_identifier,
            12 => &self.interchange_control_version_number,
            13 => &self.interchange_control_number,
            14 => &self.acknowledgement_requested,
            15 => &self.test_indicator,
            16 => &self.subelement_separator,
            _ => panic!("Invalid index"),
        }
    }
}

pub struct InterchangeSender {
    pub qualifier: Element,
    pub id: Element,
}

pub struct InterchangeReceiver {
    pub qualifier: Element,
    pub id: Element,
}

pub struct AuthorizationInformation {
    pub qualifier: Element,
    pub information: Element,
}
