/// <reference types="@fastly/js-compute" />
import { env } from 'fastly:env'

import * as Session from './session'
import * as Recommender from './recommender'
import * as MetAPI from './met-api'
import TagInsertStream from './stream-transform'
import { hasSynth, synthResponse, jsonResponse } from './responses'

const OBJECT_URL_PATTERN = /^\/art\/collection\/search\/(\d+)\/?$/
const INJECTED_HTML = `<link rel='stylesheet' href="/fastly/style" /><script src="/fastly/script" async defer></script>`

addEventListener('fetch', event => event.respondWith(handleRequest(event)))

async function handleRequest (event) {
  const req = event.request
  const reqUrl = new URL(req.url)
  const isDocReq = req.headers.get('sec-fetch-dest') === 'document'
  let resp

  Session.initFrom(req)

  // Filter requests that have unexpected methods.
  if (!['GET', 'HEAD', 'PURGE'].includes(req.method)) {
    return new Response('This method is not allowed', { status: 405 })
  }

  // If the request is for an object page, record the ID in the user's browsing history
  const objectURLMatch = reqUrl.pathname.match(OBJECT_URL_PATTERN)
  if (objectURLMatch) {
    Session.recordObjectViewed(objectURLMatch[1])
  }

  if (hasSynth(reqUrl.pathname)) {
    resp = synthResponse(reqUrl.pathname)
  } else if (reqUrl.pathname == '/fastly/recommend') {
    const startTime = performance.now();
    const recommendations = await Recommender.getSuggestions(
      Session.getHistory(),
      Number.parseInt(reqUrl.searchParams.get('offset')),
      10
    )
    const recommenderTime = performance.now() - startTime;
    const objectPromises = await Promise.allSettled(
      recommendations.map(recID => MetAPI.getObjectDetail(recID))
    )
    const objects = objectPromises
      .filter(obj => obj.status === 'fulfilled')
      .map(obj => obj.value)
    resp = jsonResponse({ objects, recommenderTime })
  } else {
    const beReq = new Request(req)
    if (isDocReq) {
      beReq.headers.delete('accept-encoding')
    }
    const beResp = await fetch(beReq, { backend: 'met' })
    const isHtmlResp = beResp.headers
      .get('content-type')
      .startsWith('text/html')

    // For HTML responses, add the client script
    if (isDocReq && isHtmlResp) {
      const newBodyStream = beResp.body.pipeThrough(
        new TagInsertStream(INJECTED_HTML)
      )
      resp = new Response(newBodyStream, {
        status: 200,
        headers: beResp.headers
      })
    } else {
      resp = beResp
    }
  }

  // Set session cookie and retarget any upstream cookies
  if (Session.getSetCookie())
    resp.headers.set('set-cookie', Session.getSetCookie())

  // Add Fastly debug
  resp.headers.set(
    'fastly-debug',
    `svcVer=${env(
      'FASTLY_SERVICE_VERSION'
    )}, user=${Session.getUserID()}, objs=${Session.getHistory()}`
  )

  // Remove extraneous HTTP metadata
  resp.headers.delete('server')
  resp.headers.delete('x-cdn')
  resp.headers.delete('x-linfo')
  resp.headers.delete('x-powered-by')
  resp.headers.delete('x-vercel-id')
  resp.headers.delete('permissions-policy')
  resp.headers.delete('link')

  return resp
}
