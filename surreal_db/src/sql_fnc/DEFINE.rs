use crate::db::DB;

pub static USER_SESSION_TIME: u8 = 12;
pub static ADMIN_SEEION_TIME: u8 = 24;

//
//
// db.use_ns("namespace").use_db("database").await?;
//
//     db.query(
//         "
//         DEFINE TABLE user SCHEMALESS
//          PERMISSIONS FOR
//             CREATE, SELECT WHERE $auth,
//             FOR UPDATE, DELETE WHERE created_by = $auth;
//          DEFINE FIELD name ON TABLE user TYPE string;
//          DEFINE FILED user_name ON TABLE user TYPE string;
//          DEFINE FILED cpid on table user TYPE string;
//
//
//
//
//         DEFINE TABLE admin
//         PERMISSIONS FOR
//             CREATE, SELECT WHERE $auth,
//             FOR UPDATE, DELETE WHERE created_by = $auth;
//     DEFINE FIELD name ON TABLE admin TYPE string;
//     DEFINE FILED user_name ON TABLE admin TYPE string;
//     DEFINE FIELD name ON TABLE user TYPE string;
//     DEFINE FILED cpid on table admin TYPE string;
//     DEFINE FIELD created_by ON TABLE admin VALUE $auth READONLY;
//     DEFINE SCOPE IF NOT EXISTS  admin SESSION {ADMIN_SEEION_TIME}h
//	      SIGNUP ( CREATE admin SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
//	      SIGNIN ( SELECT * FROM admin WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )");

//     DEFINE INDEX unique_name ON TABLE user FIELDS name UNIQUE;
//     DEFINE ACCESS account ON DATABASE TYPE RECORD
// 	SIGNUP ( CREATE user SET name = $name, pass = crypto::argon2::generate($pass) )
// 	SIGNIN ( SELECT * FROM user WHERE name = $name AND crypto::argon2::compare(pass, $pass) )
// 	DURATION FOR TOKEN 15m, FOR SESSION 12h
// ;",
//     )

pub async fn define_scope_admin() -> surrealdb::Result<()> {
    let db = DB.clone();
    db.use_ns("private_infer")
        .use_db("admin")
        .await
        .expect("1245");
    let sql = "
DEFINE TABLE admin TYPE ANY SCHEMALESS
	PERMISSIONS
		FOR select, create
			WHERE $auth
		FOR update, delete
			WHERE created_by = $auth
;
DEFINE FIELD name ON user TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD user_name ON user TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD pass ON user TYPE string
	PERMISSIONS FULL
;


DEFINE ACCESS admin ON DATABASE TYPE RECORD
	SIGNUP ( CREATE user:cpid SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
	DURATION FOR TOKEN 10m, FOR SESSION 6h
;

";
    //  let sql = format!("
    //  DEFINE SCOPE IF NOT EXISTS  admin SESSION {ADMIN_SEEION_TIME}h
    // SIGNUP ( CREATE admin SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
    // SIGNIN ( SELECT * FROM admin WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )");
    db.use_ns("users").use_db("test").await.expect("todo :msg ");
    let query_result = db.query(sql).await?;
    println!("{sql}");
    dbg!(query_result);
    Ok(())
    // if err_is_ok {
    //     let query_result = query_result.unwrap();
    //     println!("query executed successfully");
    //     Ok(query_result)
    // } else {
    //     let err = query_result.unwrap_err();
    //     Err(err)
    // }
}

pub async fn define_scope_user() -> surrealdb::Result<()> {
    let db = DB.clone();
    db.use_ns("users").use_db("test").await.expect("12098t5");
    let sql = "
DEFINE TABLE user TYPE ANY SCHEMALESS
	PERMISSIONS
		FOR select, create
			WHERE $auth
		FOR update, delete
			WHERE created_by = $auth
;
DEFINE FIELD name ON user TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD user_name ON user TYPE string
	PERMISSIONS FULL
;
DEFINE FIELD pass ON user TYPE string
	PERMISSIONS FULL
;


DEFINE ACCESS user ON DATABASE TYPE RECORD
	SIGNUP ( CREATE user:cpid SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
	DURATION FOR TOKEN 15m, FOR SESSION 12h
;


";
    //    let sql = format!("
    //    DEFINE SCOPE IF NOT EXISTS  user SESSION {USER_SESSION_TIME}h
    //   SIGNUP ( CREATE user SET name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass), is_admin = $is_admib  )
    // SIGNIN ( SELECT * FROM user WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )");

    let query_result = db.query(sql).await?;
    println!("{sql}");
    dbg!(query_result);
    Ok(())
    // let err_is_ok = query_result.is_ok();
    // if err_is_ok {
    //     let query_result = query_result.unwrap();
    //     println!("query executed successfully");
    //     Ok(query_result)
    // } else {
    //     let err = query_result.unwrap_err();
    //     Err(err)
    // }
}

// TODO: user scope function

// pub async fn define_scope_user() {
//     let db = DB.clone();
//     let sql = ""
// }
