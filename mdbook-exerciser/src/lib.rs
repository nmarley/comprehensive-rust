thoth io isaipd gnipa sipd gioa oipcnans ipdg pioasi dg a ipc apspdina i 
get
   thoth io ias ipodginopa npc ipa sipdg ipoas ipodg 
get
 a  id oi3 io ipa iopc ipa opid iopa 
a. id o3 i aopis dop ipoa cad iao 
an i ioioe io 
id
id
  oaiosdign aps ipc poaisdn iga iopsdg a
en 
 a. 8 83 8948n9 3in34io i 
idd
i 3 8a9 d89ga d

id
di
 
  iooia nicop aios dng a
nc a id 
  38 niaiosd iog aiosdg in 

id
id
id  adg

id
d i a. e

id
id
id io3 op pioasd ipg ipaosd pgioa inosdg pia iodkjjn ac
ane ioa iopcnas dgaoisgd i ino i 

search
search
saea c aoidoiasio dg inas podiiacnipasp dg ipoas nipdg iap poci ianps digp aiopsd iop e

search
      google 
find c a  

gind
c a f io
  google **oogoel(gind. find e score 38 9jo niaosd iog nioas dgi)((. score oi ioeopi a oinsdgian iosc i a
en cioa siod ingoan sidg iao 
en ioa io 
n 
c n

  iaios diog anio ico aiosd igoas dg
io
c. get o io 38 a 89sdg inas nioc io. find score oi 38 u n

scoire
c.     ifn d83n ia id ioa 

gy
  score 893 8nan sidg ioasi ondg oa 
th
th
  score 389 89asd89g8a9sdg
th
tht
  score 893 89 aisd gioa oisdg a
th 
  ffind 38n aso io aod 
 en ioc a

en ca 
     th th 

// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use log::{info, trace};
use pulldown_cmark::{Event, Parser, Tag};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

const FILENAME_START: &str = "<!-- File ";
const FILENAME_END: &str = " -->";

pub fn process(output_directory: &Path, input_contents: &str) -> anyhow::Result<()> {
    let parser = Parser::new(input_contents);

    // Find a specially-formatted comment followed by a code block, and then call `write_output`
    // with the contents of the code block, to write to a file named by the comment. Code blocks
    // without matching comments will be ignored, as will comments which are not followed by a code
    // block.
    let mut next_filename: Option<String> = None;
    let mut current_file: Option<File> = None;
    for event in parser {
        trace!("{:?}", event);
        match event {
            Event::Html(html) => {
                let html = html.trim();
                if html.starts_with(FILENAME_START) && html.ends_with(FILENAME_END) {
                    next_filename = Some(
                        html[FILENAME_START.len()..html.len() - FILENAME_END.len()]
                            .to_string(),
                    );
                    info!("Next file: {:?}:", next_filename);
                }
            }
            Event::Start(Tag::CodeBlock(x)) => {
                info!("Start {:?}", x);
                if let Some(filename) = &next_filename {
                    let full_filename = output_directory.join(filename);
                    info!("Opening {:?}", full_filename);
                    if let Some(directory) = full_filename.parent() {
                        create_dir_all(directory)?;
                    }
                    current_file = Some(File::create(full_filename)?);
                    next_filename = None;
                }
            }
            Event::Text(text) => {
                info!("Text: {:?}", text);
                if let Some(output_file) = &mut current_file {
                    output_file.write(text.as_bytes())?;
                }
            }
            Event::End(Tag::CodeBlock(x)) => {
                info!("End   {:?}", x);
                current_file = None;
            }
            _ => {}
        }
    }

    Ok(())
}
