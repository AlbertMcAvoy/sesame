import { inject } from '@angular/core';
import { Router } from '@angular/router';
import { AuthService } from '../services/auth/auth.service';

export const authGuard = () => {
  const auth = inject(AuthService);
  const router = inject(Router);

  if(!auth.isLoggedIn) {
    router.navigateByUrl('/login')
    return false
  }
  return true
};

export const authLoginGuard = () => {
  const auth = inject(AuthService);
  const router = inject(Router);

  if(auth.isLoggedIn) {
    router.navigateByUrl('/')
    return false
  }
  return true
};
