/*
 * Copyright (c) 2017 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use gtk::{
    Button,
    ButtonExt,
    ContainerExt,
    EditableSignals,
    Entry,
    EntryExt,
    Inhibit,
    Label,
    WidgetExt,
    Window,
    WindowType,
};
use gtk::Orientation::{Horizontal, Vertical};
use relm::{Component, ContainerWidget, Relm, RemoteRelm, Widget};

use self::CounterMsg::*;
use self::Msg::*;
use self::TextMsg::*;

#[derive(Clone)]
struct TextModel {
    content: String,
}

#[derive(Msg)]
enum TextMsg {
    Change,
}

struct Text {
    input: Entry,
    label: Label,
    vbox: gtk::Box,
}

impl Widget<TextMsg> for Text {
    type Container = gtk::Box;
    type Model = TextModel;

    fn container(&self) -> &Self::Container {
        &self.vbox
    }

    fn model() -> TextModel {
        TextModel {
            content: String::new(),
        }
    }

    fn update(&mut self, event: TextMsg, model: &mut TextModel) {
        match event {
            Change => {
                model.content = self.input.get_text().unwrap().chars().rev().collect();
                self.label.set_text(&model.content);
            },
        }
    }

    fn view(relm: RemoteRelm<TextMsg>, _model: &TextModel) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let input = Entry::new();
        vbox.add(&input);

        let label = Label::new(None);
        vbox.add(&label);

        connect!(relm, input, connect_changed(_), Change);

        Text {
            input: input,
            label: label,
            vbox: vbox,
        }
    }
}

#[derive(Clone)]
struct Model {
    counter: i32,
}

#[derive(Msg)]
enum CounterMsg {
    Decrement,
    Increment,
}

struct Counter {
    counter_label: Label,
    vbox: gtk::Box,
}

impl Widget<CounterMsg> for Counter {
    type Container = gtk::Box;
    type Model = Model;

    fn container(&self) -> &Self::Container {
        &self.vbox
    }

    fn model() -> Model {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, event: CounterMsg, model: &mut Model) {
        let label = &self.counter_label;

        match event {
            Decrement => {
                model.counter -= 1;
                label.set_text(&model.counter.to_string());
            },
            Increment => {
                model.counter += 1;
                label.set_text(&model.counter.to_string());
            },
        }
    }

    fn view(relm: RemoteRelm<CounterMsg>, _model: &Model) -> Self {
        let vbox = gtk::Box::new(Vertical, 0);

        let plus_button = Button::new_with_label("+");
        vbox.add(&plus_button);

        let counter_label = Label::new("0");
        vbox.add(&counter_label);

        let minus_button = Button::new_with_label("-");
        vbox.add(&minus_button);

        connect!(relm, plus_button, connect_clicked(_), Increment);
        connect!(relm, minus_button, connect_clicked(_), Decrement);

        Counter {
            counter_label: counter_label,
            vbox: vbox,
        }
    }
}

#[derive(Msg)]
enum Msg {
    Quit,
}

struct Win {
    _counter1: Component<Model, CounterMsg, gtk::Box>,
    _counter2: Component<Model, CounterMsg, gtk::Box>,
    _text: Component<TextModel, TextMsg, gtk::Box>,
    window: Window,
}

impl Widget<Msg> for Win {
    type Container = Window;
    type Model = ();

    fn container(&self) -> &Self::Container {
        &self.window
    }

    fn model() -> () {
        ()
    }

    fn update(&mut self, event: Msg, _model: &mut ()) {
        match event {
            Quit => gtk::main_quit(),
        }
    }

    fn view(relm: RemoteRelm<Msg>, _model: &()) -> Win {
        let window = Window::new(WindowType::Toplevel);

        let hbox = gtk::Box::new(Horizontal, 0);

        let counter1 = hbox.add_widget::<Counter, _, _>(&relm);
        let counter2 = hbox.add_widget::<Counter, _, _>(&relm);
        let text = hbox.add_widget::<Text, _, _>(&relm);
        window.add(&hbox);

        window.show_all();

        connect!(relm, window, connect_delete_event(_, _) (Some(Quit), Inhibit(false)));

        Win {
            _counter1: counter1,
            _counter2: counter2,
            _text: text,
            window: window,
        }
    }
}

fn main() {
    Relm::run::<Win>().unwrap();
}
