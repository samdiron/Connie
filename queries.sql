// define tables 
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


// define scopes / access

DEFINE SCOPE admin  SESSION 24h // ON DATABASE TYPE RECORD
	  SIGNUP ( CREATE admin SET id = $cpid ,name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
	  SIGNIN ( SELECT * FROM admin WHERE id = $cpid AND crypto::argon2::compare(pass, $pass) )
	  //DURATION FOR TOKEN 10m, FOR SESSION 6h
  ;

DEFINE SCOPE user //SESSION 24h //ON DATABASE TYPE RECORD
	  SIGNUP ( CREATE user SET id = $cpid ,name = $name, user_name = $user_name, pass = crypto::argon2::generate($pass) )
	  SIGNIN ( SELECT * FROM user WHERE id = $cpid AND crypto::argon2::compare(pass, $pass) )
	  //DURATION FOR TOKEN 15m, FOR SESSION 12h
  ;
