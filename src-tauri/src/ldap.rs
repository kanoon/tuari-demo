use ldap3::{LdapConn, LdapConnSettings};
use ldap3::result::Result;

fn check_credentials(ldap_url: &str, bind_dn: &str, password: &str) -> Result<bool> {
    let settings = LdapConnSettings::new();
    let mut ldap = LdapConn::with_settings(settings, ldap_url)?;

    match ldap.simple_bind(bind_dn, password)?.success() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn check_ldap() -> Result<()> {
    // Example user credentials and LDAP server details
    let ldap_url = "";
    let username = "";
    let password = "";
    let bind_dn = format!("cn={},dc=your_domain,dc=com", username);

    // Check credentials
    let result = check_credentials(ldap_url, &bind_dn, password)?;
    println!("Authentication result {}",result);
    Ok(())
}
