const COOKIE_NAME = 'fastly'
const BROWSE_HISTORY_LENGTH = 5

let sessionData, reqCookies

export const initFrom = req => {
  reqCookies = new Map(
    (req.headers.get('cookie') || '').split(/\s*;\s*/).map(str => {
      const [name, ...rest] = str.split('=')
      return [name, rest.join('=')]
    }, {})
  )
  sessionData = reqCookies.has(COOKIE_NAME)
    ? JSON.parse(decodeURIComponent(reqCookies.get(COOKIE_NAME)))
    : { id: generateRandomID(), history: [] }
}

export const recordObjectViewed = objID => {
  const idNum = Number(objID)
  const prev = sessionData.history.filter(existingID => existingID !== idNum)
  sessionData.history = [...prev, idNum].slice(-1 * BROWSE_HISTORY_LENGTH)
}

export const getUserID = () => sessionData.id

export const getHistory = () => [...sessionData.history]

export const getSetCookie = () => {
  const strSess = encodeURIComponent(JSON.stringify(sessionData))
  if (!reqCookies.has(COOKIE_NAME) || strSess !== reqCookies.get(COOKIE_NAME)) {
    return `${COOKIE_NAME}=${strSess}; path=/; HttpOnly; SameSite=Strict`
  }
}

function generateRandomID () {
  const timestamp = Date.now().toString(36)
  const randomPart = Math.random().toString(36).substring(2, 10)
  return timestamp + randomPart
}
