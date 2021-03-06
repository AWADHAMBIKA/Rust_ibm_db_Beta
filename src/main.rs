// c_from_rust/src/main.rs

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes)]

use ::ibm_db::SQLConnect;
use ibm_db::{SQLAllocHandle, SQL_HANDLE_DBC, SQL_HANDLE_ENV, SQL_SUCCESS, SQLFreeHandle, SQLSMALLINT, SQLExecDirect, SQL_NTS, SQLCHAR, SQLSetConnectAttr, SQL_ATTR_AUTOCOMMIT, SQL_AUTOCOMMIT_ON, SQLPOINTER, SQL_HANDLE_STMT, SQL_NO_DATA_FOUND, SQLFreeStmt, SQL_UNBIND, SQL_RESET_PARAMS, SQL_CLOSE};

/*#[link(name = "mystrlen")]
extern "C" {
    fn mystrlen(str: *const c_char) -> c_uint;
}
*/
/*fn safe_mystrlen(str: &str) -> Option<u32> {
    let c_string = match CString::new(str) { 
        Ok(c) => c, 
        Err(_) => return None 
    };

    unsafe { 
        Some(mystrlen(c_string.as_ptr())) 
    } 
}*/

fn main() {
    let dsn = "dashdb4";
    let uid = "ts5612";
    let pwd = "mar@2019";
    let conn = connect(
        dsn.parse().unwrap(),
       // "db2admin".parse().unwrap(),
        uid.parse().unwrap(),
        //"gr8tcode!".parse().unwrap()
        pwd.parse().unwrap()
    );
    unsafe{
        if conn == 0{
            println!("Connection failed...");
            std::process::exit(-1);
        }
        dml(conn);
        closeConnection(conn);
    }


}

fn connect(mut dsn : String, mut uid: String, mut pwd: String) -> i32 {
    /*let c_string = CString::new("C From Rust").expect("failed");
    let count = unsafe { 
        mystrlen(c_string.as_ptr()) 
    };*/
    //let c2_string:CCHAR = CCHAR::new("DSN=dashdb;DATABASE=RS22DDS2;hostname=rs22.rocketsoftware.com;PORT=3720;UID=ts5612;PWD=mar@2019;").expect("failed");

    unsafe {
        let mut hdbc = 0;
        //let henv:*mut SQLHANDLE = ptr::null_mut();

        //let mut dsn = String::new();
        //dsn.push_str("dashdb");
        //let mut uid = String::new();
        //uid.push_str("db2admin");
        //let mut pwd = String::new();
        //pwd.push_str("gr8tcode!");
        let mut cliRC;
        let mut out = 0;
        cliRC = SQLAllocHandle(SQL_HANDLE_ENV as i16,
                               0,
                               &mut out);
        if cliRC != SQL_SUCCESS as i16 {
            println!("--ERROR while allocating the environment handle. Status: {}", cliRC);
            return 0;
        }

        cliRC = SQLAllocHandle(SQL_HANDLE_DBC as i16,
                               out,
                               &mut hdbc);
        if cliRC != SQL_SUCCESS as i16 {
            println!("--ERROR while getting hdbc. Status: {}", cliRC);
            return 0;
        }
        println!("Connecting to database: {} .........", dsn);
        SQLConnect(
            hdbc
            ,
            dsn.as_mut_ptr()
            ,
            dsn.len() as i16
            ,
            uid.as_mut_ptr()
            ,
            uid.len() as i16
            ,
            pwd.as_mut_ptr()
            ,
            pwd.len() as i16
        );

        SQLSetConnectAttr(hdbc, SQL_ATTR_AUTOCOMMIT as i32, SQL_AUTOCOMMIT_ON as SQLPOINTER, SQL_NTS);


        let mut hstmt = 0;

        //println!("Allocating Statement Handle");
        cliRC = SQLAllocHandle(SQL_HANDLE_STMT as SQLSMALLINT,
                               hdbc,
                               &mut hstmt);
        if cliRC != SQL_SUCCESS as i16 {
            //println!("--ERROR while getting statement. Status: {}", cliRC);
            return 0;
        }
        println!("Connected Successfully to database: {}", dsn);
        return hstmt;
    }
}

pub unsafe fn dml(conn: i32){
        let mut cliRC;

        println!("Dropping table if it exists.....");
        let mut query = "DROP TABLE TEST";
        let mut stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
        println!("Dropping Table Result: {}",cliRC);

        println!("Creating table.....");
        query = "create table test(Col3 VARCHAR(7))";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
        println!("Creating Table Result: {}",cliRC);

        println!("Inserting Data.....");
        query = "INSERT INTO TEST VALUES ('Binit')";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        /*cliRC = SQLPrepare(hstmt,stmt,query.as_bytes().len() as i32);
        println!("Insert Prepare Statement Result: {}",cliRC);
        cliRC = SQLBindParameter(hstmt,
                                1,
                                    SQL_PARAM_INPUT as i16,
                                    SQL_C_CHAR as i16,
                                    SQL_VARCHAR as i16,
                                        7,
                                            0,
                                                "Binit".as_bytes().as_ptr() as SQLPOINTER,
                                        7,
                                    7 as *mut i32);
        println!("Insert Bind Parameter Result: {}",cliRC);*/

        cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
        println!("Inserting Data Result: {}",cliRC);


     /*   cliRC = SQLSetStmtAttr(hstmt,SQL_ATTR_USE_LOAD_API as i32,SQL_USE_LOAD_INSERT as SQLPOINTER,0);
        println!("Statement Attr Set Result: {}",cliRC);

        let db2LoadStruct = pLoadStruct
        cliRC = SQLSetStmtAttr(hstmt,SQL_ATTR_LOAD_INFO as i32,SQL_USE_LOAD_INSERT as SQLPOINTER,0);
        println!("Statement Attr Set Result: {}",cliRC);

        cliRC = SQLExecute(hstmt);
        println!("Fetching Data Result: {}",cliRC);*/

        println!("Fetching Data.....");
        query = "SELECT * FROM TEST";
        stmt = query.as_bytes().as_ptr() as *mut SQLCHAR;
        cliRC = SQLExecDirect(conn, stmt, query.as_bytes().len() as i32);
        if cliRC == SQL_NO_DATA_FOUND as i16{
            println!("No Data in Table");
        } else{
            println!("Fetching Data Result: {}",cliRC);
        }

        //After Select, Handle is missing so recreating the same.
      /*  println!("Allocating Statement Handle");
        cliRC = SQLAllocHandle(SQL_HANDLE_STMT as SQLSMALLINT,
                               hdbc,
                               &mut conn);
        if cliRC!= SQL_SUCCESS as i16 {
            println!("--ERROR while getting hdbc. Status: {}",cliRC);
            return;
        }*/
    }
pub unsafe fn closeConnection(conn:i32){
    println!("Disconnecting ...");
    SQLFreeHandle(SQL_HANDLE_STMT as i16, conn);
    SQLFreeStmt(conn, SQL_UNBIND as u16);
    SQLFreeStmt(conn, SQL_RESET_PARAMS as u16);
    SQLFreeStmt(conn, SQL_CLOSE as u16);
    println!("Disconnected Successfully.");
    /*let dbInfoBuf = core::ptr::null_mut();
    let outlen:*mut SQLSMALLINT = core::ptr::null_mut();
    let supported:*mut SQLUSMALLINT = core::ptr::null_mut();
    /* check to see if SQLGetInfo() is supported */
    SQLGetFunctions(conn, SQL_API_SQLGETINFO as u16,
                    supported);
    if supported == SQL_TRUE as *mut u16
    {
        SQLGetInfo(hdbc, SQL_DATA_SOURCE_NAME as u16,
                   dbInfoBuf, 255, outlen);
        let cstr = CStr::from_ptr(dbInfoBuf as *const _).to_string_lossy();
        println!("dbInfoBuf has value: {}", !dbInfoBuf.is_null());
        println!("DSN name: {}", cstr);
    }*/
}

    /*println!("c_string's length is {}", count);
    println!("c_string's length is {:?}", safe_mystrlen("C From Rust"));*/

