// export async function getUserInfo() {
//   const res = await api.getUser()
//   const { id, username, profile, roles, currentRole } = res.daka || {}
//   return {
//     id,
//     username,
//     avatar: profile?.avatar,
//     nickName: profile?.nickName,
//     gender: profile?.gender,
//     address: profile?.address,
//     email: profile?.email,
//     roles,
//     currentRole,
//   }
// }

// export async function getPermissions() {
//   let asyncPermissions = []
//   try {
//     const res = await api.getRolePermissions()
//     asyncPermissions = res?.daka || []
//   } catch (error) {
//     console.error(error)
//   }
//   return basePermissions.concat(asyncPermissions)
// }
