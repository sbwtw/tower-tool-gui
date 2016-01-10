
extern crate gtk;
extern crate time;

use gtk::traits::*;
use gtk::signal::Inhibit;

use std::char;
use std::rc::Rc;
use std::cell::{RefCell};
use std::ops::{Deref, DerefMut};
use std::thread;
use std::process;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver};


struct TowerPlugin {
    main_window: gtk::Window,
    user_name: gtk::Entry,
    user_password: gtk::Entry,
    overtime_at_list: gtk::Entry,
    entry_mo: gtk::TextView,
    entry_tu: gtk::TextView,
    entry_we: gtk::TextView,
    entry_th: gtk::TextView,
    entry_fr: gtk::TextView,
    entry_m1: gtk::TextView,
    entry_m2: gtk::TextView,
    receiver: RefCell<Option<(gtk::TextBuffer, Receiver<String>)>>,
}

impl TowerPlugin {
    fn submit_weekly(&self) {

        let start_iter = self.entry_mo.get_buffer().unwrap().get_start_iter();
        let mut end_iter = self.entry_mo.get_buffer().unwrap().get_start_iter();
        end_iter.forward_to_end();

        println!("{}", start_iter.get_text(&end_iter).unwrap());

        let mo = TowerPlugin::get_text_view_string(&self.entry_mo);
        println!("{}", mo);
    }

    fn get_text_view_string(buf: &gtk::TextView) -> String {

        let start = buf.get_buffer().unwrap().get_start_iter();
        let mut end = buf.get_buffer().unwrap().get_start_iter();
        end.forward_to_end();

        start.get_text(&end).unwrap()
    }

    fn calculate_duration(tm: &time::Tm) -> String {

        let mut start_time = time::now();
        start_time.tm_min = 30;
        start_time.tm_hour = 17;

        let duration = *tm - start_time;

        let mins: i64 = (duration.num_minutes() + 10) / 30;
        let hour: f64 = mins as f64 / 2.0;

        if hour > 0.0 {
            hour.to_string()
        } else {
            "0".to_owned()
        }
    }

    fn submit_overtime(&mut self) {

        //let a = Arc::new(Mutex::new(self.user_name));

        //let mut d = a.lock().unwrap();
        //thread::spawn(move || {
            //d.set_text("aa");
        //});

        //let work = thread::spawn(move || {
            //let output = process::Command::new("/home/Downloads/tower-tool")
                                            //.arg("-e")
                                            //.arg("a")
                                            //.arg("-p")
                                            //.arg("p")
                                            //.output()
                                            //.unwrap();

            //println!("exit, is: {}, {}", output.status.code().unwrap(), String::from_utf8_lossy(&output.stdout));

            //t.deref().borrow_mut().deref().import_from_cr();
            //o.user_name.set_text("a");
            //self.user_name.set_text("aaa");
        //});

        //let (rx, tx) = channel();
    }

    fn import_from_cr(&self) {

    }

    fn gui_main() {

        if gtk::init().is_err() {
            panic!("gtk init fail");
        }

        let label_mo = gtk::Label::new("Mo").unwrap();
        label_mo.set_halign(gtk::Align::Start);
        let label_tu = gtk::Label::new("Tu").unwrap();
        label_tu.set_halign(gtk::Align::Start);
        let label_we = gtk::Label::new("We").unwrap();
        label_we.set_halign(gtk::Align::Start);
        let label_th = gtk::Label::new("Th").unwrap();
        label_th.set_halign(gtk::Align::Start);
        let label_fr = gtk::Label::new("Fr").unwrap();
        label_fr.set_halign(gtk::Align::Start);
        let label_m1 = gtk::Label::new("M1").unwrap();
        label_m1.set_halign(gtk::Align::Start);
        let label_m2 = gtk::Label::new("M2").unwrap();
        label_m2.set_halign(gtk::Align::Start);

        let entry_mo = gtk::TextView::new().unwrap();
        let entry_tu = gtk::TextView::new().unwrap();
        let entry_we = gtk::TextView::new().unwrap();
        let entry_th = gtk::TextView::new().unwrap();
        let entry_fr = gtk::TextView::new().unwrap();
        let entry_m1 = gtk::TextView::new().unwrap();
        let entry_m2 = gtk::TextView::new().unwrap();

        let btn_submit = gtk::Button::new_with_label("Submit").unwrap();
        let btn_import = gtk::Button::new_with_label("Import").unwrap();

        let btns_layout = gtk::Box::new(gtk::Orientation::Horizontal, 5).unwrap();
        btns_layout.set_halign(gtk::Align::Center);
        btns_layout.add(&btn_import);
        btns_layout.add(&btn_submit);

        let weekly_layout = gtk::Box::new(gtk::Orientation::Vertical, 5).unwrap();
        weekly_layout.set_border_width(15);
        weekly_layout.add(&label_mo);
        weekly_layout.add(&entry_mo);
        weekly_layout.add(&label_tu);
        weekly_layout.add(&entry_tu);
        weekly_layout.add(&label_we);
        weekly_layout.add(&entry_we);
        weekly_layout.add(&label_th);
        weekly_layout.add(&entry_th);
        weekly_layout.add(&label_fr);
        weekly_layout.add(&entry_fr);
        weekly_layout.add(&label_m1);
        weekly_layout.add(&entry_m1);
        weekly_layout.add(&label_m2);
        weekly_layout.add(&entry_m2);
        weekly_layout.add(&gtk::Separator::new(gtk::Orientation::Horizontal).unwrap());
        weekly_layout.add(&btns_layout);

        let overtime_date = gtk::Label::new("Date").unwrap();
        overtime_date.set_halign(gtk::Align::Start);
        let overtime_reason = gtk::Label::new("Reason").unwrap();
        overtime_reason.set_halign(gtk::Align::Start);
        let overtime_finish_time = gtk::Label::new("Finish time").unwrap();
        overtime_finish_time.set_halign(gtk::Align::Start);
        let overtime_at_bodies = gtk::Label::new("@People list").unwrap();
        overtime_at_bodies.set_halign(gtk::Align::Start);

        let current_tm = time::now();
        let current_date = time::strftime("%F", &current_tm).unwrap();
        let current_time = time::strftime("%R", &current_tm).unwrap();

        let overtime_date_entry = gtk::Entry::new().unwrap();
        overtime_date_entry.set_text(&current_date);
        let overtime_reason_entry = gtk::Entry::new().unwrap();
        overtime_reason_entry.set_text("加班登记");
        let overtime_finish_time_entry = gtk::Entry::new().unwrap();
        overtime_finish_time_entry.set_text(&current_time);
        let overtime_at_list = gtk::Entry::new().unwrap();
        overtime_at_list.set_text("@耀华 @叶凯胜 @夏彬");

        let btn_overtime_submit = gtk::Button::new_with_label("Submit").unwrap();

        let btns_overtime_layout = gtk::Box::new(gtk::Orientation::Vertical, 5).unwrap();
        btns_overtime_layout.add(&btn_overtime_submit);

        let overtime_layout = gtk::Box::new(gtk::Orientation::Vertical, 5).unwrap();
        overtime_layout.set_border_width(15);
        overtime_layout.add(&overtime_date);
        overtime_layout.add(&overtime_date_entry);
        overtime_layout.add(&overtime_reason);
        overtime_layout.add(&overtime_reason_entry);
        overtime_layout.add(&overtime_finish_time);
        overtime_layout.add(&overtime_finish_time_entry);
        overtime_layout.add(&overtime_at_bodies);
        overtime_layout.add(&overtime_at_list);
        overtime_layout.add(&gtk::Separator::new(gtk::Orientation::Horizontal).unwrap());
        overtime_layout.add(&btns_overtime_layout);

        let label_weekly = gtk::Label::new("Weekly").unwrap();
        let label_overtime = gtk::Label::new("Overtime").unwrap();

        let notebook = gtk::NoteBook::new().unwrap();
        notebook.append_page(&weekly_layout, Some(&label_weekly));
        notebook.append_page(&overtime_layout, Some(&label_overtime));

        let label_uname = gtk::Label::new("Username").unwrap();
        let label_upwd = gtk::Label::new("Password").unwrap();
        let user_name = gtk::Entry::new().unwrap();
        let user_password = gtk::Entry::new().unwrap();
        user_password.set_visibility(false);

        let centeral_layout = gtk::Box::new(gtk::Orientation::Vertical, 5).unwrap();
        centeral_layout.set_border_width(8);
        centeral_layout.add(&label_uname);
        centeral_layout.add(&user_name);
        centeral_layout.add(&label_upwd);
        centeral_layout.add(&user_password);
        centeral_layout.add(&notebook);

        let main_window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();
        main_window.add(&centeral_layout);
        main_window.set_title("Tower weekly");
        main_window.set_window_position(gtk::WindowPosition::Center);
        main_window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let tower_plugin = Rc::new(RefCell::new(TowerPlugin{
            main_window: main_window,
            user_name: user_name,
            user_password: user_password,
            overtime_at_list: overtime_at_list,
            entry_mo: entry_mo,
            entry_tu: entry_tu,
            entry_we: entry_we,
            entry_th: entry_th,
            entry_fr: entry_fr,
            entry_m1: entry_m1,
            entry_m2: entry_m2,
            receiver: RefCell::new(None),
        }));

        let tower = tower_plugin.clone();
        btn_overtime_submit.connect_clicked(move |_| {
            TowerPlugin::submit_overtime(tower.deref().borrow_mut().deref_mut());
        });

        let tower = tower_plugin.clone();
        btn_submit.connect_clicked(move |_| {
            TowerPlugin::submit_weekly(tower.deref().borrow().deref());
        });

        let tower = tower_plugin.clone();
        btn_import.connect_clicked(move |_| {
            TowerPlugin::import_from_cr(tower.deref().borrow().deref());
        });

        tower_plugin.deref().borrow().main_window.show_all();
    }
}

fn main() {
    TowerPlugin::gui_main();

    gtk::main();
}

#[test]
fn test_calculate_duration() {
    let mut time = time::now();
    time.tm_min = 30;
    time.tm_hour = 19;
    assert_eq!(TowerPlugin::calculate_duration(&time), "2");

    time.tm_min = 20;
    assert_eq!(TowerPlugin::calculate_duration(&time), "1.5");

    time.tm_min = 21;
    assert_eq!(TowerPlugin::calculate_duration(&time), "2");

    time.tm_min = 50;
    assert_eq!(TowerPlugin::calculate_duration(&time), "2");

    time.tm_min = 51;
    assert_eq!(TowerPlugin::calculate_duration(&time), "2.5");

    time.tm_hour = 0;
    assert_eq!(TowerPlugin::calculate_duration(&time), "0");
}
