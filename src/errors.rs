// This file is part of oxideNews-common
//
// Copyright © 2017 red-oxide Developers
//
// his program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//

//! All possible errors that can be thrown by this library.

use ron::de::Error as DeError;
use ron::ser::Error as SerError;
use rss::Error as RssError;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
/// Errors from RSS and the Database
pub enum Error
{
    /// Errors from the rss crate.
    Rss(RssError),
    /// Errors from the io library.
    Io(IoError),
    /// Errors from ron serialize.
    Ser(SerError),
    /// Errors from ron deserialize.
    De(DeError),
}

impl StdError for Error
{
    fn description(&self) -> &str
    {
        match *self {
            Error::Rss(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Ser(ref err) => err.description(),
            Error::De(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&StdError>
    {
        match *self {
            Error::Rss(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Ser(ref err) => Some(err),
            Error::De(ref err) => Some(err),
        }
    }
}

impl fmt::Display for Error
{
    fn fmt(&self,
           f: &mut fmt::Formatter)
        -> fmt::Result
    {
        match *self {
            Error::Rss(ref err) => err.fmt(f),
            Error::Io(ref err) => err.fmt(f),
            Error::Ser(ref err) => err.fmt(f),
            Error::De(ref err) => err.fmt(f),
        }
    }
}

impl From<RssError> for Error
{
    fn from(err: RssError) -> Error
    {
        Error::Rss(err)
    }
}

impl From<IoError> for Error
{
    fn from(err: IoError) -> Error
    {
        Error::Io(err)
    }
}

impl From<SerError> for Error
{
    fn from(err: SerError) -> Error
    {
        Error::Ser(err)
    }
}

impl From<DeError> for Error
{
    fn from(err: DeError) -> Error
    {
        Error::De(err)
    }
}
