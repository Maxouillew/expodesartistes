function createAdminAuthStore() {
  let isAuthenticated = $state(false);

  return {
    get isAuthenticated() {
      return isAuthenticated;
    },
    login() {
      isAuthenticated = true;
    },
    logout() {
      isAuthenticated = false;
    },
  };
}

export const adminAuth = createAdminAuthStore();
