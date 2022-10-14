use std::io;

fn main() {
    loop {
        println!("This program is for setting up Zinc-Nickel Plating as a");
        println!("low hydrogen embrittlement alkaline electrodeposited");

        println!("Enter a value to review selections below: ");
        println!("Enter 1 to calculate tank size in gallons");
        println!("Enter 2 to see amount of chemicals required in gallons and pounds");
        println!(
            "Enter 3 to see Application of Zinc-Nickel Plating and Trivalent Chromate Conversion"
        );
        println!("Enter 4 to see contacts from chemical vendors");
        println!("Enter 5 to see Quality Assurance Requirements");
        println!("Enter 6 to see Equipment Requirements");
        println!("Enter 7 to see Maintance Requirements");
        println!("Enter 8 to see cost of chemicals");
        println!("Enter 9 to see met specifications");
        let mut user_value = String::new();
        io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line.");
        let user_value: i32 = user_value.trim().parse().expect("Please type a number!");

        if user_value == 9 {
            println!("Product used are as follows:");
            println!("Zinc-Nickel Plating Solution: Dispol IZ-C17");
            println!("Trivalent Chromate Conversion Coating: Dispol IZ-264");
            println!("These two products are good for the following specifications:");
            println!("AMS 2417, BAC 5637, MIL-PRF-32660, U.S. Air Force 201027456");
        } else if user_value == 1 {
            
            println!("\nPlease enter height dimension in inches of the tank: ");
            let mut tank_height = String::new();
            io::stdin().read_line(&mut tank_height).expect("Failed to read line");
            let tank_height: f32 = tank_height.trim().parse().expect("Please type a number!");
            
            println!("Please enter width dimension in inches of the tank: ");
            let mut tank_width = String::new();
            io::stdin().read_line(&mut tank_width).expect("Failed to read line");
            let tank_width: f32 = tank_width.trim().parse().expect("Please type a number!");

            println!("Please enter length dimension in inches of the tank: ");
            let mut tank_length = String::new();
            io::stdin().read_line(&mut tank_length).expect("Failed to read line");
            let tank_length: f32 = tank_length.trim().parse().expect("Please type a number!");

            println!("Please enter filled depth dimension in inches of the tank: ");
            let mut tank_depth = String::new();
            io::stdin().read_line(&mut tank_depth).expect("Failed to read line");
            let tank_depth: f32 = tank_depth.trim().parse().expect("Please type a number!");

            let full_volume: f32 = tank_height*tank_width*tank_length;
            let less_volume: f32 = (tank_height - tank_depth)*tank_width*tank_length;
            let tank_volume: f32 = (full_volume-less_volume)* 0.004329;

            println!("{}, {}, {}", tank_height, tank_length, tank_width);
            println!("Total Tank Capacity volume is {} gallons ", (full_volume*0.004329).floor());
            println!("Filled Tank volume is {} gallons ", tank_volume.floor());

        } else if user_value == 2 {
            println!("Please enter solution: 1 for Dispol IZ-C17 or 2 for Dispol IZ-264");

            println!("Require chemicals for Dispol IZ-C17 are as follows: ");
            println!("Dipsol IZ-C17+MS : # gallons");
            println!("Dipsol Sodium Hydroxide: # pounds");
            println!("Dipsol NZ-777: # gallons");
            println!("Dipsol IZ-C17+B: # gallons");
            println!("Dipsol F-0529: # gallons");

            println!("Require chemicals for Dispol IZ-264 are as follows: ");
            println!("Dispol IZ-264: # gallons");
            println!("Dispol IZ-264 T: # gallons");
        } else if user_value == 3 {
            println!("Application of Dispol IZ-C17 with Dispol IZ-264 is as follows: ");
            println!("Step 1: Degrease - Solvent Clean to remove grease and oil");
            println!("Step 2: Pre Plate Stress Relief - bake for 5 - 10 hours at 375 F");
            println!("Step 3: Grit Blast - 120 grit aluminum oxide at 40 - 60 psi per MIL-STD-1504 or equivalent");
            println!("Step 4: Clean - remove grit blast residue with clean compressed air. Plate within (1) hour after grit blast");
            println!("Step 5: Mask - as required");
            println!("Step 6: Rinse and Clean - water rinse for 90 seconds and scrub part with clean nylon bristle brush");
            println!("Step 7: Inspect - water-break free surface");
            println!("Step 8: Acid Activate - immerse part in 0.1% HCl for 30 seconds max or until gassing observed");
            println!("Step 9: Rinse - water rinse for minimum of 90 seconds");
            println!("Step 10: Plating - plate at 28 - 56 ASF for 25 minutes to produce 0.3 - 0.6 mils at 68 - 78 F");
            println!("Step 11: Rinse - water rinse for minimum of 90 seconds");
            println!("Step 12: Demask - keep product wet");
            println!("Step 13: Rinse - water rinse for minimum of 30 seconds");
            println!("Step 14: Acid Activate - immerse in 0.1% HCl for 30 seconds max");
            println!("Step 15: Rinse - water rinse for minimum of 30 seconds");
            println!("Step 16: Trivalent Chromate Conversion Coating - Immersion in solution for 30 to 120 seconds at 61 to 89 F");
            println!("Step 17: Air Time - Hold parts in Air  for 30 seconds to complete Chromate Conversion Reaction");
            println!("Step 18: Rinse - water rinse for minimum of 90 seconds");
            println!("Step 19: Rinse - hot water rinse at 160 - 190 F for 10 seconds");
            println!("Step 20: Drying - apply forced air drying");
            println!("Step 21: Demask/Unrack - if not performed before");
            println!("Step 22: Inspect - measure thickness and nickel co-deposition");
            println!("Step 23: Bake - bake within 4 hours of completion of plating at 375F for 23 hours minimum");
        } else if user_value == 4 {
            println!("Dispol of America");
            println!("34005 Schoolcraft St, Livonia, Mi 48150");
            println!("phone: 734-261-0633");
            println!("website: https://www.dipsolamerica.com/");
            println!("Contacts: ");
            println!("Chris Kolar");
            println!("Title: Technical Service Representative");
            println!("email: ckolar@dipsolamerica.com");
            println!("phone: 734-812-1021");
            println!("John Szczypka");
            println!("Title: NA Sales Manager");
            println!("email: JSzczypka@dipsolamerica.com");
            println!("phone: 734-261-0633");
            println!("Brenntag Mid-South, Inc");
            println!("website: https://www.brenntagmid-south.com");
            println!("Contact: ");
            println!("Tammy Hoang");
            println!("Title: Account Manager - Durham District");
            println!("email: thoang@brenntag.com");
            println!("phone: 270-832-5292");
        } else if user_value == 5 {
            println!("Quality Assurance Provisions");
            println!("Required Tests are as followed: ");
            println!("Thickness - test shall be in accordance with ASTM B568, ASTM E376, or method acceptable to purchaser");
            println!("Adhesion - bend test until rupture");
            println!("Finish Quality - visual examination");
            println!("Composition - shall be determined by X-Ray Fluorescence on production parts");
            println!("Corrosion Resistance - ASTM B117 for 96 hours");
            println!("Hydrogen Embrittlement - ASTM F519 and photomicrographs");
            println!("Processing bath chemistry - weekly testing");
            println!("Paint Adhesion - 14 days air dry and then dry and wet paint adhesion test");
            println!("Quality Inspection Criteria");
            println!("Finish Quality, thickness, and composition shall be performed on every lot");
            println!("Finish Quality: on every part");
            println!("Thickness: based on sampling plan");
            println!("Composition: based on sampling plan");
        } else if user_value == 6 {
            println!("Equipment Requirements are as follows: ");
            println!("Tanks - non-metallic such as PVC, CPVC, or polyethylene");
            println!("Anodes - Nickel 200 anodes have 99.0% nickel");
            println!("Pumps/Filtration - pump at least 3X turnover per hour, filter elements rated at 20 - 50 micrometers");
            println!("Carbon Treatment - treatment external to plating tank");
            println!("Carbonate Removal -  cooling equipment to precipitate the carbonates");
            println!("Toshi Cell - 500 ml Toshi Cell");
            println!("Amp-Hour Meter - total accumulated amp-hours to determine additions");
            println!("Do not use air agitation due to it will cause rapid rise in carbonates");
        } else if user_value == 7 {
            println!("Solution Maintenance are as follows: ");
            println!("Solution operation is less than 85 F");
            println!("300 grams per liter of IZ-C17+MS");
            println!("130 grams per liter of Sodium Hydroxide");
            println!("10 milliliters per liter of IZ-C17+B");
            println!("Component Concentrations should be as follows: ");
            println!("Sodium Hydroxide: 120 - 140 grams/Liter");
            println!("Zinc: 6 - 10 grams/Liter");
            println!("Nickel: 1.7 - 2.3 grams/Liter");
            println!("Addjustment of F-0529 should be as follows: ");
            println!("Toshi cell test");
            println!("500 milliliters sample with 2 milliliters of F-0529");
            println!("plate a toshi cell at 4 amps for 20 mintues at 68 -83 F");
            println!("plate a toshi cell as above with a sample as is");
            println!("compare cells and adjust at 4 milliliters of F-0529 per liter");
            println!("cell shall be 0.45 to 0.53 mils at 1 cm from high current density edge with 14.5 to 16.0 % nickel");
            println!("cell shall be 0.20 to 0.28 mils at 5 cm from high current density edge with 14.5 to 16.0 % nickel");
            println!("if nickel concentration is low, add IZ-C17+Ni");
            println!("if thickness is too thick, stop sodium hydroxide or zinc additions or reduce the plating bath temperature");
            println!("if thickness is too thin, increase zinc or sodium hydroxide or increase temperature of the plating bath");
            println!("deposit is acceptable if nickel is between 12 to 18 %");
            println!("Trivalent Chromium Conversion Coating Solution");
            println!("pH: 4.0 to 4.4, use sodium hydroxide or nitric acid as required");
            println!("temperature: 73 to 86 F");
            println!("Trivalent Chromium: 1.2 - 2.6 grams/Liter, use IZ-264T to adjust, filtered sample before testing");
            println!("Cobalt: 2.0 - 4.0 grams/Liter, use IZ-264 to adjust");
            println!(
            "Trace metals: Cr6+ <1 mg/L, Zn <5 g/L, Cu < 3 mg/L, Nickel < 20 mg/L, Fe < 100 mg/L"
        );
        } else if user_value == 8 {
            println!("Chemical Cost");
            println!("Dipsol IZ-C17+ Zinc-Nickel Component Pricing: ");
            println!("Dipsol IZ-C17+MS: $65.00 per gal");
            println!("Dipsol IZ-C17+Ni: $103.50 per gal");
            println!("Dipsol IZ-C17+B: $167.00 per gal");
            println!("Dipsol NZ-777: $21.66 per gal");
            println!("Dipsol F-0529: $42.00 per gal");
            println!("Sodium Hydroxide: $1.86 per pound");

            println!("Dipsol IZ-264 Trivalent Passivate Component Pricing: ");
            println!("Dipsol IZ-264: $47.00 per gal");
            println!("Dipsol IZ-264-T: $47.00 per gal");

            println!("Cost for Dipsol IZ-C17+ is: ");
            println!("Cost for Dipsol IZ-264 is: ");
        } else {
            println!("You entered an invalid respond.");
        }

        println!(
            "\nDo you want to review another selection for plating Zn-Ni: Y for yes and N for no"
        );
        let mut user_continue = String::new();
        io::stdin()
            .read_line(&mut user_continue)
            .expect("Failed to read line.");
        let user_continue = user_continue.trim();
        if user_continue == "N" {
            break println!("Thanks for using the app, Bye");
        }
    }
}
