/*
 *  Me TV — It's TV for me computer.
 *
 *  A GTK+/GStreamer client for watching and recording DVB.
 *
 *  Copyright © 2017, 2018  Russel Winder
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use gtk;

use gstreamer;
use gstreamer::prelude::*;

pub struct GStreamerEngine{
    playbin: gstreamer::Element,
    video_element: gstreamer::Element,
    video_widget: gtk::Widget,
}

impl GStreamerEngine {
    pub fn new() -> GStreamerEngine {
        let playbin = gstreamer::ElementFactory::make("playbin", "playbin").expect("Failed to create playbin element");
        // BEGIN COPIED CODE
        // This code copied from https://github.com/sdroege/gstreamer-rs/blob/master/examples/src/bin/gtksink.rs
        // and then slightly amended.
        let (video_element, video_widget) = if let Some(gtkglsink) = gstreamer::ElementFactory::make("gtkglsink", None) {
            let glsinkbin = gstreamer::ElementFactory::make("glsinkbin", None).unwrap();
            glsinkbin
                .set_property("sink", &gtkglsink.to_value())
                .unwrap();
            let widget = gtkglsink.get_property("widget").unwrap();
            (glsinkbin, widget.get::<gtk::Widget>().unwrap())
        } else {
            let sink = gstreamer::ElementFactory::make("gtksink", None).unwrap();
            let widget = sink.get_property("widget").unwrap();
            (sink, widget.get::<gtk::Widget>().unwrap())
        };
        // END COPIED CODE
        GStreamerEngine {
            playbin,
            video_element,
            video_widget,
        }
    }
}