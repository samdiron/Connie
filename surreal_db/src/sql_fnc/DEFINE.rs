use crate::db::DB;

pub async fn define_scope_admin() -> surrealdb::Result<()> {
    let db = DB.clone();
    db.use_ns("private_infer")
        .use_db("admin")
        .await
        .expect("1245");
    let sql_table = "
    DEFINE TABLE admin TYPE ANY SCHEMALESS
	    PERMISSIONS
		    FOR select, create
			    WHERE $auth,
		    FOR update, delete
			    WHERE created_by = $auth
    ;
    DEFINE FIELD name ON admin TYPE string
	    PERMISSIONS FULL
    ;
    DEFINE FIELD user_name ON admin TYPE string
	    PERMISSIONS FULL
    ;
    DEFINE FIELD pass ON admin TYPE string
	    PERMISSIONS FULL
    ;
    DEFINE FIELD cpid ON admin TYPE string
        PERMISSIONS FULL
    ;

";
    db.use_ns("private_infer")
        .use_db("admin")
        .await
        .expect("todo :msg ");

    //TODO: format this query after version 2.0.0;
    //to ACCESS
    let scope_query = "
    DEFINE SCOPE admin  SESSION 24h // ON DATABASE TYPE RECORD
	    SIGNUP ( CREATE admin SET cpid = $cpid ,name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
	    SIGNIN ( SELECT * FROM admin WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )
	    //DURATION FOR TOKEN 10m, FOR SESSION 6h
    ;
";
    let table_query_result = db.query(sql_table).await.expect("could not run a db query");
    dbg!(table_query_result);
    let scope_query_result = db
        .query(scope_query)
        .await
        .expect("could not run a db query: scope");
    dbg!(scope_query_result);
    Ok(())
}

pub async fn define_scope_user() -> surrealdb::Result<()> {
    let db = DB.clone();
    db.use_ns("users")
        .use_db("test")
        .await
        .expect("could not use ns/db");
    let table_query = "
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
    DEFINE FIELD pass ON user TYPE string
        PERMISSIONS FULL
    ;
";
    let scope_query = "
    DEFINE SCOPE user //SESSION 24h //ON DATABASE TYPE RECORD
	    SIGNUP ( CREATE user SET cpid = $cpid ,name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
	    SIGNIN ( SELECT * FROM user WHERE cpid = $cpid AND crypto::argon2::compare(pass, $pass) )
	    //DURATION FOR TOKEN 15m, FOR SESSION 12h
    ;
";

    let query_result = db
        .query(table_query)
        .await
        .expect("could not run a db query:table");
    dbg!(query_result);
    let scope_query_result = db
        .query(scope_query)
        .await
        .expect("could not run a db query:scope");
    dbg!(scope_query_result);
    Ok(())
}

// TODO: user scope function

// pub async fn define_scope_user() {
//     let db = DB.clone();
//     let sql = ""
// }
