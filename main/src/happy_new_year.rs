use crate::HappyNewYearArgs;
use crossterm::{
    cursor, execute,
    style::{self, Color, Stylize},
    terminal::{self, ClearType},
    QueueableCommand,
};
use rand::Rng;
use std::io::Write;
use std::{io::stdout, thread, time::Duration};

pub fn happy_new_year(args: HappyNewYearArgs) -> eyre::Result<()> {
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::Clear(ClearType::All),
        cursor::Hide,
        cursor::MoveTo(0, 0)
    )?;

    // Get terminal size
    let (columns, rows) = terminal::size().unwrap_or((80, 24));

    // Define decorative arts
    let year_2025_ascii = r#"
                 --------                 
              ------==------              
           ------=#%%%%#+-------          
       ------=+#%%%%%%%%%%#+=------       
    ------=*#%%%%%%%%%%%%%%%%#*=------    
 ------=#%%%%%%%%%%%%%%%%%%%%%%%%#=------ 
----+#%%%%%%%%*+++++%%++=++*%%%%%%%%#+----
--=#%%%%%%%%%*.  ..+%+..  .*%%%%%%%%%%#=--
--=#%%%%%%%%#-. . -##. .  +%%%%%%%%%%%#=--
--=#%%%%%%%%=.  .:*%:.  .-%#+*%%%%%%%%#=--
--=#%%%%%%%*.   .+%=.   :#%*++#%%%%%%%#=--
--=#%%%%%%%:.   -%*. . .+%#++++%%%%%%%#=--
--=#%%%%%%=.   .%#-    -#%+++++*%%%%%%#=--
--=#%%%%%+.   .*%=   .:*%%#+++++#%%%%%#=--
--=#%%%%#:.   =%*.   .+%##%#+++++#%%%%#=--
--=#%%%#-.  .-#%.   .-%#++%%*++++*%%%%#=--
--=#%%%+..  .*%-    .%#*++*%#+++++#%%%#=--
--=#%%#: . .=%+. . .*%*++++*%*+++++#%%#=--
---=+#-. ..:#*:.  .=%%#+++++#%*++++*#+=---
 ----:.    ##-.   -#%%%*++++*%%++=------- 
    :... .=%+... .*%%%%#+++++*#=------    
       ..-%#.   .=%%%%%%#++=-------       
          #-... .=#%%%%#+-------          
              ..:---==------              
                 --------                 
        "#;

    let decorative_art_2 = r#"
                ##########                
              ####%%%%%#####              
            ####%%%%%%###%####            
           ###%%%%%%###%%%#####           
         ###%%%%%%%##%%%###%%%###         
       ####%%%%%%##%%%%##%%%%%%%###       
     ####%%%%%%###%%%###%%%%%%%%%####     
   ####%%%%%%###%%%###%**#%%%%%%%%%####   
 ####%%%%%%###%%%###%%=::::-=%%%%%%%%#### 
###%%%%%%%###%%###%#=+#%%%*-::+%%%%%%%%###
##%%%%%%%%##%%###%#==--:::=##-:=%%%%%%%%##
##%%%%%%%%##%%###%%%%%%%#=:-#+-:%%%%%%%%##
##%%%%%%%%%##%%#######%%%=:-#*-:#%%%%%%%##
###%%%%%%%%%###%%%%%%#%*-:-##=:-%%%%%%%###
 ####%%%%%%%%%######%#-:-*%#-:-%%%%%%#### 
   ####%%%%%%%%%%%%#-:-+%#-:-*%%%%%####   
     ####%%%%%%%%#=::=%#=::+%%%%%####     
       ###%%%%%%+-:=#%=-:=%%%%%%###       
         ###%%+-:=#%+-:=#%%%%%####        
           #+-:-#%+-:-#%%%%%####          
            --*%#-:-#%%%%%####            
              #-:-+%%%%%####              
                =*########                
        "#;

    let decorative_art_1 = r#"
██████╗  ██████╗ ██████╗ ███████╗
╚════██╗██╔═████╗╚════██╗██╔════╝
 █████╔╝██║██╔██║ █████╔╝███████╗
██╔═══╝ ████╔╝██║██╔═══╝ ╚════██║
███████╗╚██████╔╝███████╗███████║
╚══════╝ ╚═════╝ ╚══════╝╚══════╝
                                  
"#;

    // Countdown
    for i in (0..=3).rev() {
        stdout.queue(terminal::Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(columns / 2 - 2, rows / 2))?;
        stdout.queue(style::PrintStyledContent(
            format!("{}!", i).with(Color::Yellow).bold(),
        ))?;
        stdout.flush()?;
        thread::sleep(Duration::from_secs(1));
    }

    // Fireworks and animations
    let fireworks = vec!["🎆✨🎆✨🎆", "✨🎇✨🎇✨", "🎆🎇🎆🎇🎆", "✨🎆✨🎆✨"];
    for _ in 0..20 {
        stdout.queue(terminal::Clear(ClearType::All))?;

        // Place decorative_art_1
        let art_1_x = ((columns / 4).saturating_sub(7) as u16).max(0);
        let art_1_y = ((rows / 2).saturating_sub(6) as u16).max(0);

        stdout.queue(cursor::MoveTo(art_1_x, art_1_y))?;
        stdout.queue(style::PrintStyledContent(
            decorative_art_1.to_string().with(Color::Blue).bold(),
        ))?;
        stdout.flush()?; // Flush after queuing the ASCII art

        // Place decorative_art_1

        // Place decorative_art_2
        stdout.queue(cursor::MoveTo(
            ((3 * columns) / 4).saturating_sub(7) as u16,
            (rows / 2).saturating_sub(6) as u16,
        ))?;
        stdout.queue(style::PrintStyledContent(
            decorative_art_2.to_string().with(Color::Magenta).bold(),
        ))?;

        // Draw fireworks randomly
        for _ in 0..5 {
            let firework = fireworks[rand::thread_rng().gen_range(0..fireworks.len())];
            let x = rand::thread_rng().gen_range(0..columns);
            let y = rand::thread_rng().gen_range(0..rows);
            let color = match rand::thread_rng().gen_range(0..4) {
                0 => Color::Blue,
                1 => Color::Blue,
                2 => Color::Blue,
                3 => Color::Blue,
                _ => Color::Blue,
            };
            stdout.queue(cursor::MoveTo(x as u16, y as u16))?;
            stdout.queue(style::PrintStyledContent(firework.to_string().with(color)))?;
        }

        // Display 2025 ASCII art in the center
        let lines: Vec<&str> = year_2025_ascii.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            stdout.queue(cursor::MoveTo(
                (columns / 2).saturating_sub(line.len() as u16 / 2),
                (rows / 2).saturating_sub(lines.len() as u16 / 2) + i as u16,
            ))?;
            stdout.queue(style::PrintStyledContent(
                line.to_string().with(Color::Green).bold(),
            ))?;
        }

        stdout.flush()?;
        thread::sleep(Duration::from_millis(100));
    }

    // Final greeting
    let message = format!("🎉 {} 🎉", args.message);
    let message_len = message.chars().count() as u16;
    let x = (columns / 2).saturating_sub(message_len / 2) as u16;
    let y = (rows - 2) as u16;
    stdout.queue(cursor::MoveTo(x, y))?;
    stdout.queue(style::PrintStyledContent(message.with(Color::White).bold()))?;

    execute!(stdout, cursor::Show)?;
    Ok(())
}
