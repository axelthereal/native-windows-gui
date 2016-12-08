/*!
    Errors and exceptions that can be raise by the crate
*/
/*
    Copyright (C) 2016  Gabriel Dubé

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::fmt;

use events::Event;

/**
    Error class that regroups errors generated by the system
*/
#[derive(Clone, PartialEq)]
pub enum SystemError {
    SystemClassCreation,
    WindowCreationFail,
    UiCreation,
}

impl SystemError {
    fn translate(&self) -> String {
        use low::other_helper::get_system_error;

        let (code, code_txt) = unsafe{ get_system_error() };
        let tr = match self {
            &SystemError::SystemClassCreation => format!("Failed to create a system class for a control"),
            &SystemError::WindowCreationFail => format!("Failed to create a system window for a control"),
            &SystemError::UiCreation => format!("The system could not initialize the Ui"),
        };

        format!("{}.\nID {:?} - {}", tr, code, code_txt)
    }
}

impl fmt::Debug for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.translate())
    }
}

/**
    Error class that regroup errors generated by NWG
*/
#[derive(Clone, PartialEq)]
pub enum Error {
    KeyExists,
    KeyNotFound,
    BadType,
    BorrowError,
    EventNotSupported(Event),
    ControlRequired,
    ControlOrResourceRequired,
    ControlInUse,
    Unimplemented,
    System(SystemError)
}

impl Error {
    fn translate(&self) -> String {

        match self {
            &Error::KeyExists => format!("The same key already exists in the UI"),
            &Error::KeyNotFound => format!("The key was not found in the ui"),
            &Error::BadType => format!("The key exists in the Ui, but the type requested did not match the type of the underlying object"),
            &Error::BorrowError => format!("The Ui element was already borrowed"),
            &Error::EventNotSupported(ref e) => format!("The event of type {:?} is not supported on this control", e),
            &Error::ControlRequired => format!("The key passed to the command must identify a control"),
            &Error::ControlOrResourceRequired => format!("The key passed to the command must identify a control or a resource", ),
            &Error::ControlInUse => format!("Impossible to modify the control, it is currently in use."),
            &Error::Unimplemented => format!("Feature not yet implemented"),
            &Error::System(ref e) => format!("A system error was raised: {:?}", e),
        }

    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.translate())
    }
}