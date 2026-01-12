export default defineNuxtRouteMiddleware((to) => {
  const authCookie = useCookie('auth_token')

  if (!authCookie.value && to.path !== '/login') {
    return navigateTo('/login')
  }

  if (authCookie.value && to.path === '/login') {
    return navigateTo('/')
  }
})
