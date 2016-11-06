extern crate clap;
extern crate chrono;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::*;
use std::str::*;

use clap::{Arg, App};
use chrono::*;

pub const DATE_FORMAT: &'static str = "%Y-%m-%d";

#[allow(unused_variables)]
fn main() {

    let all_tickers :Vec<&str> = vec!["AA", "AAPL", "ABC", "ABI", "ABKFQ", "ABT", "ACAS", "ACE", "ACN", "ACS", "A", "ADBE", "ADI", "ADM", "ADP", "ADSK", "AEE", "AEP", "AES", "AET", "AFL", "AGN", "AIG", "AIV", "AIZ", "AKAM", "ALL", "ALTR", "ALXN", "AMAT", "AMD", "AMGN", "AMP", "AMT", "AMZN", "AN", "ANF", "ANR", "AON", "APA", "APC", "APD", "APH", "APOL", "ARG", "ASH", "ATI", "AVB", "AVP", "AVY", "AXP", "AYE", "AZO", "BAC", "BA", "BAX", "BBBY", "BBT", "BBY", "BC", "BCR", "BDK", "BDX", "BEAM", "BEN", "BF.B", "BHI", "BIG", "BIIB", "BJS", "BK", "BLK", "BLL", "BMC", "BMS", "BMY", "BRCM", "BRK.B", "BRLI", "BSC", "BSX", "BTU", "BUD", "BWA", "BXP", "CA", "CAG", "CAH", "CAM", "CAT", "CB", "CBE", "CBG", "CBS", "CBSH", "CCE", "CCI", "CCL", "CCMO", "C", "CCT", "CEG", "CELG", "CERN", "CFC+A", "CF", "CFN", "CHK", "CHRW", "CI", "CIEN", "CINF", "CIT", "CL", "CLF", "CLX", "CMA", "CMCSA", "CME", "CMG", "CMI", "CMS", "CNP", "CNX", "COF", "COG", "COH", "COL", "COP", "COST", "COV", "CPB", "CPWR", "CRM", "CSC", "CSCO", "CSX", "CTAS", "CTL", "CTSH", "CTX", "CTXS", "CVC", "CVG", "CVH", "CVS", "CVX", "D", "DD", "DDR", "DDS", "DE", "DELL", "DF", "DFS", "DGX", "DHI", "DHR", "DISCA", "DIS", "$DJI", "DLTR", "DNB", "DNR", "DO", "DOV", "DOW", "DPS", "DRI", "DTE", "DTV", "DUK", "DVA", "DV", "DVN", "DYN", "EA", "EBAY", "ECL", "ED", "EDS", "EFX", "EIX", "EK", "EL", "EMC", "EMN", "EMR", "EOG", "EP", "EQ", "EQR", "EQT", "ERTS", "ESRX", "ESV", "ETFC", "ETN", "ETR", "EW", "EXC", "EXPD", "EXPE", "FAST", "F", "FCX", "FDO", "FDX", "FE", "FFIV", "FHN", "FII", "FIS", "FISV", "FITB", "FLIR", "FLR", "FLS", "FMCC", "FMC", "FNMA", "FOSL", "FRX", "FSLR", "FTI", "FTR", "GAS", "GCI", "GD", "GE", "GENZ", "GGP", "GILD", "GIS", "GLD", "GLW", "GM", "GME", "GNW", "GOOG", "GPC", "GPS", "GR", "GS", "GT", "GWW", "HAL", "HAR", "HAS", "HBAN", "HCBK", "HCN", "HCP", "HD", "HES", "HIG", "HNZ", "HOG", "HON", "HOT", "HPCCP", "HP", "HPQ", "HRB", "HRL", "HRS", "HSP", "HST", "HSY", "HUM", "IACI", "IBM", "ICE", "IFF", "IGT", "INTC", "INTU", "IP", "IPG", "IR", "IRM", "ISRG", "ITT", "ITW", "IVZ", "IYR", "JAVA", "JBL", "JCI", "JCP", "JDSU", "JEC", "JNJ", "JNPR", "JNS", "JNY", "JOY", "JPM", "JWN", "KBH", "K", "KEY", "KFT", "KG", "KIM", "KLAC", "KMB", "KMI", "KMX", "KO", "KR", "KSS", "L", "LEG", "LEHMQ", "LEN", "LH", "LIFE", "Lists", "LIZ", "LLL", "LLTC", "LLY", "LM", "LMT", "LNC", "LO", "LOW", "LRCX", "LSI", "LTD", "LUK", "LUV", "LXK", "LYB", "MA", "MAR", "MAS", "MAT", "MBI", "MCD", "MCHP", "MCK", "MCO", "M", "MDP", "MDT", "MET", "MHP", "MHS", "MI", "MIL", "MJN", "MKC", "ML4T-000", "ML4T-001", "ML4T-002", "ML4T-003", "ML4T-004", "ML4T-005", "ML4T-006", "ML4T-007", "ML4T-008", "ML4T-009", "ML4T-010", "ML4T-011", "ML4T-012", "ML4T-013", "ML4T-014", "ML4T-015", "ML4T-016", "ML4T-017", "ML4T-018", "ML4T-019", "ML4T-020", "ML4T-021", "ML4T-022", "ML4T-023", "ML4T-024", "ML4T-025", "ML4T-026", "ML4T-027", "ML4T-028", "ML4T-029", "ML4T-030", "ML4T-031", "ML4T-032", "ML4T-033", "ML4T-034", "ML4T-035", "ML4T-036", "ML4T-037", "ML4T-038", "ML4T-039", "ML4T-040", "ML4T-041", "ML4T-042", "ML4T-043", "ML4T-044", "ML4T-045", "ML4T-046", "ML4T-047", "ML4T-048", "ML4T-049", "ML4T-050", "ML4T-051", "ML4T-052", "ML4T-053", "ML4T-054", "ML4T-055", "ML4T-056", "ML4T-057", "ML4T-058", "ML4T-059", "ML4T-060", "ML4T-061", "ML4T-062", "ML4T-063", "ML4T-064", "ML4T-065", "ML4T-066", "ML4T-067", "ML4T-068", "ML4T-069", "ML4T-070", "ML4T-071", "ML4T-072", "ML4T-073", "ML4T-074", "ML4T-075", "ML4T-076", "ML4T-077", "ML4T-078", "ML4T-079", "ML4T-080", "ML4T-081", "ML4T-082", "ML4T-083", "ML4T-084", "ML4T-085", "ML4T-086", "ML4T-087", "ML4T-088", "ML4T-089", "ML4T-090", "ML4T-091", "ML4T-092", "ML4T-093", "ML4T-094", "ML4T-095", "ML4T-096", "ML4T-097", "ML4T-098", "ML4T-099", "ML4T-100", "ML4T-101", "ML4T-102", "ML4T-103", "ML4T-104", "ML4T-105", "ML4T-106", "ML4T-107", "ML4T-108", "ML4T-109", "ML4T-110", "ML4T-111", "ML4T-112", "ML4T-113", "ML4T-114", "ML4T-115", "ML4T-116", "ML4T-117", "ML4T-118", "ML4T-119", "ML4T-120", "ML4T-121", "ML4T-122", "ML4T-123", "ML4T-124", "ML4T-125", "ML4T-126", "ML4T-127", "ML4T-128", "ML4T-129", "ML4T-130", "ML4T-131", "ML4T-132", "ML4T-133", "ML4T-134", "ML4T-135", "ML4T-136", "ML4T-137", "ML4T-138", "ML4T-139", "ML4T-140", "ML4T-141", "ML4T-142", "ML4T-143", "ML4T-144", "ML4T-145", "ML4T-146", "ML4T-147", "ML4T-148", "ML4T-149", "ML4T-150", "ML4T-151", "ML4T-152", "ML4T-153", "ML4T-154", "ML4T-155", "ML4T-156", "ML4T-157", "ML4T-158", "ML4T-159", "ML4T-160", "ML4T-161", "ML4T-162", "ML4T-163", "ML4T-164", "ML4T-165", "ML4T-166", "ML4T-167", "ML4T-168", "ML4T-169", "ML4T-170", "ML4T-171", "ML4T-172", "ML4T-173", "ML4T-174", "ML4T-175", "ML4T-176", "ML4T-177", "ML4T-178", "ML4T-179", "ML4T-180", "ML4T-181", "ML4T-182", "ML4T-183", "ML4T-184", "ML4T-185", "ML4T-186", "ML4T-187", "ML4T-188", "ML4T-189", "ML4T-190", "ML4T-191", "ML4T-192", "ML4T-193", "ML4T-194", "ML4T-195", "ML4T-196", "ML4T-197", "ML4T-198", "ML4T-199", "ML4T-200", "ML4T-201", "ML4T-202", "ML4T-203", "ML4T-204", "ML4T-205", "ML4T-206", "ML4T-207", "ML4T-208", "ML4T-209", "ML4T-210", "ML4T-211", "ML4T-212", "ML4T-213", "ML4T-214", "ML4T-215", "ML4T-216", "ML4T-217", "ML4T-218", "ML4T-219", "ML4T-220", "ML4T-221", "ML4T-222", "ML4T-223", "ML4T-224", "ML4T-225", "ML4T-226", "ML4T-227", "ML4T-228", "ML4T-229", "ML4T-230", "ML4T-231", "ML4T-232", "ML4T-233", "ML4T-234", "ML4T-235", "ML4T-236", "ML4T-237", "ML4T-238", "ML4T-239", "ML4T-240", "ML4T-241", "ML4T-242", "ML4T-243", "ML4T-244", "ML4T-245", "ML4T-246", "ML4T-247", "ML4T-248", "ML4T-249", "ML4T-250", "ML4T-251", "ML4T-252", "ML4T-253", "ML4T-254", "ML4T-255", "ML4T-256", "ML4T-257", "ML4T-258", "ML4T-259", "ML4T-260", "ML4T-261", "ML4T-262", "ML4T-263", "ML4T-264", "ML4T-265", "ML4T-266", "ML4T-267", "ML4T-268", "ML4T-269", "ML4T-270", "ML4T-271", "ML4T-272", "ML4T-273", "ML4T-274", "ML4T-275", "ML4T-276", "ML4T-277", "ML4T-278", "ML4T-279", "ML4T-280", "ML4T-281", "ML4T-282", "ML4T-283", "ML4T-284", "ML4T-285", "ML4T-286", "ML4T-287", "ML4T-288", "ML4T-289", "ML4T-290", "ML4T-291", "ML4T-292", "ML4T-293", "ML4T-294", "ML4T-295", "ML4T-296", "ML4T-297", "ML4T-298", "ML4T-299", "ML4T-300", "ML4T-301", "ML4T-302", "ML4T-303", "ML4T-304", "ML4T-305", "ML4T-306", "ML4T-307", "ML4T-308", "ML4T-309", "ML4T-310", "ML4T-311", "ML4T-312", "ML4T-313", "ML4T-314", "ML4T-315", "ML4T-316", "ML4T-317", "ML4T-318", "ML4T-319", "ML4T-320", "ML4T-321", "ML4T-322", "ML4T-323", "ML4T-324", "ML4T-325", "ML4T-326", "ML4T-327", "ML4T-328", "ML4T-329", "ML4T-330", "ML4T-331", "ML4T-332", "ML4T-333", "ML4T-334", "ML4T-335", "ML4T-336", "ML4T-337", "ML4T-338", "ML4T-339", "ML4T-340", "ML4T-341", "ML4T-342", "ML4T-343", "ML4T-344", "ML4T-345", "ML4T-346", "ML4T-347", "ML4T-348", "ML4T-349", "ML4T-350", "ML4T-351", "ML4T-352", "ML4T-353", "ML4T-354", "ML4T-355", "ML4T-356", "ML4T-357", "ML4T-358", "ML4T-359", "ML4T-360", "ML4T-361", "ML4T-362", "ML4T-363", "ML4T-364", "ML4T-365", "ML4T-366", "ML4T-367", "ML4T-368", "ML4T-369", "ML4T-370", "ML4T-371", "ML4T-372", "ML4T-373", "ML4T-374", "ML4T-375", "ML4T-376", "ML4T-377", "ML4T-378", "ML4T-379", "ML4T-380", "ML4T-381", "ML4T-382", "ML4T-383", "ML4T-384", "ML4T-385", "ML4T-386", "ML4T-387", "ML4T-388", "ML4T-389", "ML4T-390", "ML4T-391", "ML4T-392", "ML4T-393", "ML4T-394", "ML4T-395", "ML4T-396", "ML4T-397", "ML4T-398", "ML4T-399", "MMC", "MMM", "MNST", "MO", "MOLX", "MON", "MOS", "MPC", "MRK", "MRO", "MS", "MSFT", "MSI", "MTB", "MTG", "MTW", "MU", "MUR", "MWV", "MWW", "MYL", "NBL", "NBR", "NCC", "NDAQ", "NE", "NEE", "NEM", "NFLX", "NFX", "NI", "NKE", "NOC", "NOV", "NOVL", "NRG", "NSC", "NSM", "NTAP", "NTRS", "NU", "NUE", "NVDA", "NVLS", "NWL", "NWSA", "NYT", "NYX", "ODP", "OI", "OKE", "OMC", "OMX", "ORCL", "ORLY", "OXY", "PAYX", "PBCT", "PBG", "PBI", "PCAR", "PCG", "PCL", "PCLN", "PCP", "PCS", "PDCO", "PEG", "PEP", "PFE", "PFG", "PG", "PGN", "PGR", "PH", "PHM", "PKI", "PLD", "PLL", "PM", "PNC", "PNW", "POM", "PPG", "PPL", "PRGO", "PRU", "PSA", "PSX", "PTV", "PWR", "PX", "PXD", "QCOM", "Q", "QEP", "QLGC", "RAI", "R", "RDC", "RF", "RHI", "RHT", "RIG", "RL", "ROH", "ROK", "ROP", "ROST", "RRC", "RRD", "RSG", "RSH", "RTN", "SAF", "SAI", "SBUX", "SCG", "SCHW", "S", "SDS", "SE", "SEE", "SGP", "SH", "SHLD", "SHW", "SIAL", "SII", "SINE_FAST", "SINE_FAST_NOISE", "SINE_SLOW", "SINE_SLOW_NOISE", "SJM", "SLB", "SLE", "SLM", "SNA", "SNDK", "SNI", "SNV", "SO", "SPG", "SPLS", "$SPX", "SPY", "SRCL", "SRE", "SSP", "STI", "STJ", "STR", "STT", "STX", "STZ", "SUN", "SVU", "SWK", "SWN", "SWY", "SYK", "SYMC", "SYY", "TAP", "T", "TDC", "TE", "TEG", "TEL", "TER", "TEX", "TGT", "THC", "TIE", "TIF", "TJX", "TLAB", "TMK", "TMO", "TRIP", "TROW", "TRV", "TSN", "TSO", "TSS", "TWC", "TWX", "TXN", "TXT", "TYC", "UIS", "UNH", "UNM", "UNP", "UPS", "URBN", "USB", "UST", "UTX", "VAR", "V", "VFC", "VIA.B", "VIAB", "$VIX", "VLO", "VMC", "VNO", "VRSN", "VTR", "VZ", "WAG", "WAMUQ", "WAT", "WB", "WDC", "WEC", "WEN", "WFC", "WFM", "WFR", "WFT", "WHR", "WIN", "WLP", "WMB", "WM", "WMT", "WPI", "WPO", "WPX", "WU", "WWY", "WY", "WYE", "WYN", "WYNN", "X", "XEL", "XL", "XLNX", "XOM", "XRAY", "XRX", "XTO", "XYL", "YHOO", "YUM", "ZION", "ZMH"];
    let all_tickers_2 = read_all_tickers();

    let matches = App::new("Quant Software")
        .version("0.1")
        .author("Marek Dudek")
        .arg(Arg::with_name("start-date").long("start-date").takes_value(true))
        .arg(Arg::with_name("end-date").long("end-date").takes_value(true))
        .arg(Arg::with_name("tickers").long("tickers").takes_value(true))
        .get_matches();

    let start_date = extract_date_with_default(matches.value_of("start-date"),
                                               NaiveDate::from_ymd(1962, 7, 5));
    let end_date = extract_date_with_default(matches.value_of("end-date"),
                                             NaiveDate::from_ymd(2131, 1, 2));
    let tickers = extract_tickers_with_default(matches.value_of("tickers"), all_tickers);

    println!("Processing for period between {} and {}.",
             start_date,
             end_date);
    println!("Tickers: {:?}", tickers);


    // fetch_data(&all_tickers);
}

pub fn extract_date_with_default(param: Option<&str>, default: NaiveDate) -> NaiveDate {
    let result: Option<Result<NaiveDate, ParseError>> =
        param.map(|s| NaiveDate::parse_from_str(s, DATE_FORMAT));
    result.map(|r| r.unwrap()).unwrap_or(default)
}

pub fn read_all_tickers() -> Vec<String> {
    let path = "/home/marek/Devel/rust/projects/quant_software/src/resources/tickers.txt";
    let file = File::open(path).expect("No tickers file");
    let buffer = BufReader::new(file);
    let lines = buffer.lines().map(|l| l.expect("cound not parse ticker line"));
    lines.collect()
}

pub fn extract_tickers_with_default<'a>(param: Option<&'a str>, default: Vec<&'a str>) -> Vec<&'a str> {
    let tokens: Option<Split<&str>> = param.map(|s| s.split(","));
    let vector: Option<Vec<&str>> = tokens.map(|ts| ts.collect::<Vec<&str>>());
    vector.unwrap_or(default)
}

#[allow(unused_variables)]
pub fn fetch_data(tickers: &Vec<&str>) {
    println!("Fetchin data");
    let path = "/usr/local/lib/python2.7/dist-packages/QSTK-0.2.8-py2.7.\
                egg/QSTK/QSData/Yahoo/GOOG.csv";
    let file = File::open(path).expect("no such file");
    let buffer = BufReader::new(file);
    let lines: Vec<String> = buffer.lines().map(|l| l.expect("could not parse")).collect();
    println!("Length of file: {}", lines.len());
}


#[cfg(test)]
mod tests {

    use super::*;
    use chrono::*;

    #[test]
    fn parsing_date_with_default() {
        let date = extract_date_with_default(Some("1962-07-05"), NaiveDate::from_ymd(1962, 7, 5));
        assert_eq!(date, NaiveDate::from_ymd(1962, 7, 5));
    }

    #[test]
    fn fetching_data() {
        fetch_data(&vec!["AA"]);
    }
}
