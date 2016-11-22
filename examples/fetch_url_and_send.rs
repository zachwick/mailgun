/**
 * rget - a wget type program in Rust
 *
 * Copyright 2016 zach wick <zach@zachwick.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

extern crate curl;
extern crate argparse;

use std::io::{stdin, stdout, Write};
use curl::easy::{Easy, Form, Transfer};

use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut url = "https://zachwick.com".to_string();
    let mut email = "zach@zachwick.com".to_string();

    let mg_key = "key-XXXYYYZZZ".to_string();
    let mg_url = "https://api.mailgun.net/v3/mg.kith.mx/messages".to_string();
    let mg_from = "zach@zachwick.com".to_string();
    let mut easy = Easy::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("a wget type program for fetching HTML");
        ap.refer(&mut url)
            .add_option(&["-u", "--url"], Store,
                        "URL to fetch");
        ap.refer(&mut email)
            .add_option(&["-e", "--email"], Store,
                        "Email address to send HTML to");
        ap.parse_args_or_exit();
    }

    easy.url(&url.trim()).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        let mut mg_handle = Easy::new();
        let mut mg_form = Form::new();
        let mut html_string = String::from_utf8_lossy(data);

        /*
        Send an email via Mailgun by doing the following cURL request:

        curl -s --user 'api:YOUR_API_KEY' \
           https://api.mailgun.net/v3/YOUR_DOMAIN_NAME/messages \
           -F from='Excited User <mailgun@YOUR_DOMAIN_NAME>' \
           -F to=YOU@YOUR_DOMAIN_NAME \
           -F to=bar@example.com \
           -F subject='Hello' \
           -F text='Testing some Mailgun awesomness!'
         */

        mg_form.part("from").contents(mg_from.as_bytes()).add();
        mg_form.part("to").contents(email.as_bytes()).add();
        mg_form.part("subject").contents(url.as_bytes()).add();
        mg_form.part("text").contents(html_string.to_string().as_bytes()).add();
        mg_handle.url(&mg_url);
        mg_handle.httppost(mg_form);
        mg_handle.username("api");
        mg_handle.password(&mg_key);
        mg_handle.perform().unwrap();

        let mg_res = mg_handle.response_code().unwrap();
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}
