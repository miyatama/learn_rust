pub trait UserDao {
    fn find_user(&self, id: i32) -> ();
}

pub trait HaveUserDao {
    type UserDao: UserDao;
    fn user_dao(&self) -> Self::UserDao;
}

pub trait UserService: HaveUserDao {
    fn get_user_by_id(&self, id: i32) -> () {
        self.user_dao().find_user(id)
    }
}

impl<T: HaveUserDao> UserService for T {}

struct RepositoryImpl<Repo: UserDao> {
    repo: Repo,
}

trait HaveUserService {
    type UserService: UserService;
    fn user_service(&self) -> Self::UserService;
}

pub struct ServiceImpl<Service: UserService> {
    service: Service,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_by_id() {
        struct MockRepository {}
        impl UserDao for MockRepository {
            fn find_user(&self, id: i32) -> () {}
        }
        struct DaoComponent {}
        impl HaveUserDao for DaoComponent {
            type UserDao = MockRepository;
            fn user_dao(&self) -> Self::UserDao {
                MockRepository {}
            }
        }

        let dao_component = DaoComponent {};
        let service = ServiceImpl {
            service: dao_component,
        };
        let user = service.service.get_user_by_id(2);
        assert_eq!(user, 3);
    }
}
