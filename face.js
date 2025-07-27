let mq = window.matchMedia('(max-width: 1024px)'),
mask;
const resultPopup = document.querySelector('#result'),
guidePopup = document.querySelector('#guide'),
askPopup = document.querySelector('#ask'),
pollPopup = document.querySelector('#poll');
function checkLogin(e) {
  Cookies.get('is_login') ? e() : $('.fs-recognition__btns .fs-recognition__upload-trigger').jajax()
}
function initFaceRecognition(t) {
  if (
    document.querySelector('.upload_file_path_key').dataset.httpUrl = t[0],
    document.querySelector('.upload_recognition_id').setAttribute('value', t[1]),
    t[0]
  ) {
    let e = document.createElement('img');
    e.src = t[0],
    pollImgUploader.makeThumb(e, !1)
  }
  faceRecognition.init(),
  new FsDropdown('.recognition-dropdown'),
  fsQuiz.init(),
  reRenderFace()
}
YES.live(
  document,
  'click',
  '.fs-recognition__bottom__title .fs-btn--primary',
  () => Hash.set('#result')
);
const guideOpenerList = document.querySelectorAll('.guide-opener'),
guideOpenerMobileList = document.querySelectorAll('.guide-opener-mobile');
function openGuidePopup(e) {
  const t = new URL(e.href);
  t.searchParams.set('target', 'face-shape-detail-container'),
  Hash.set('#guide').setAttribute('href', t)
}
guideOpenerList.forEach(
  t => {
    t.addEventListener('click', e => {
      e.preventDefault(),
      openGuidePopup(t)
    })
  }
),
guideOpenerMobileList.forEach(
  (t, o) => {
    t.addEventListener(
      'click',
      e => {
        e.preventDefault(),
        mq.matches ? openGuidePopup(t) : (
          faceShapeNav.scrollToIndex(o),
          faceShapeNav.setActiveNavItem(o)
        )
      }
    )
  }
),
guidePopup.addEventListener(
  'hash:loaded',
  () => {
    navAutoScroll('#guide'),
    window.checkFrameColorsOverflow()
  }
),
askPopup.addEventListener('hash:close', () => {
  cropClose()
}),
YES.live(
  document,
  'click',
  '.ask-popup-opener',
  function () {
    checkLogin(() => Hash.set('#ask'))
  }
),
YES.live(
  document,
  'click',
  '.poll-popup-opener',
  function (e) {
    let t;
    t = e.target.classList.contains('poll-popup-opener') ? $(e.target) : $(e.target).closest('.poll-popup-opener'),
    e.preventDefault(),
    Hash.set('#poll').setAttribute('href', t.data('href'))
  }
),
pollPopup.addEventListener(
  'hash:loaded',
  () => {
    document.querySelector('#poll .poll-dropdown') &&
    new FsDropdown('#poll .poll-dropdown')
  }
),
YES.live(
  document,
  'click',
  '.btn-my-post',
  e => {
    e.preventDefault(),
    $(e.target).jajax({
      data: {
        w: window.screen.width,
        h: window.screen.height,
        dppx: window.devicePixelRatio
      }
    }).then(
      e => {
        'ok' === e.status &&
        (
          Hash.set('#mypost'),
          new FsDropdown('#my-post-container .poll-dropdown')
        )
      }
    )
  }
);
let faceCropper,
cropResultImg,
cropResultBlob,
corpCallback,
cropMask;
const cropLayer = document.querySelector('.crop'),
cropCanvas = document.querySelector('.crop .crop-canvas'),
cropRotateBtn = document.querySelector('.crop .crop-rotate'),
cropContinueBtn = document.querySelector('.crop .crop-continue'),
cropCancelBtn = document.querySelector('.crop .crop-cancel'),
cropCloseBtn = document.querySelector('.crop .crop__close');
function cropImage(e, t) {
  e &&
  e[0] &&
  (
    cropCanvas.src = URL.createObjectURL(e[0]),
    faceCropper = new Cropper(
      cropCanvas,
      {
        aspectRatio: 0.75,
        viewMode: 1,
        zoomable: !1,
        autoCropArea: 1
      }
    ),
    corpCallback = t,
    cropOpen()
  )
}
function cropOpen() {
  cropLayer.hidden = !1,
  setTimeout(
    () => {
      cropLayer.classList.add('show'),
      lockScrollbar(!0),
      cropMask = new SiblingMask(cropLayer),
      cropMask.getMaskElem().addEventListener('click', () => cropClose())
    },
    50
  )
}
function cropClose() {
  !1 === cropLayer.hidden &&
  (
    cropLayer.classList.remove('show'),
    cropMask.remove(),
    transCall(
      cropLayer.querySelector('.crop__main'),
      () => {
        lockScrollbar(!1),
        cropLayer.hidden = !0,
        cropCanvas.removeAttribute('src'),
        faceCropper.destroy(),
        faceRecognition.inputElement.value = '',
        pollImgUploader.uploadInput.value = '',
        '#ask' === window.location.hash &&
        lockScrollbar(!0)
      }
    )
  )
}
function finishRecognitionCrop() {
  document.querySelector('#crop_result').innerHTML = '',
  document.querySelector('#crop_result').appendChild(cropResultImg),
  recognitionUploader.upload(cropResultBlob),
  document.querySelector('.face-shape-scaner').innerHTML = '',
  faceRecognition.progressElement.style.width = '5%',
  faceRecognition.loading()
}
function finishAskCrop() {
  pollImgUploader.upload(
    cropResultBlob,
    function () {
      pollImgUploader.makeThumb(cropResultImg)
    }
  )
}
cropRotateBtn.addEventListener('click', () => {
  faceCropper.rotate(90)
}),
cropContinueBtn.addEventListener(
  'click',
  () => {
    faceCropper.getCroppedCanvas({
      width: 600,
      height: 800,
      fillColor: '#fff'
    }).toBlob(
      e => {
        let t = document.createElement('img'),
        o = URL.createObjectURL(e);
        t.onload = () => {
          URL.revokeObjectURL(o)
        },
        t.src = o,
        cropResultImg = t,
        cropResultBlob = e,
        corpCallback(),
        cropClose()
      },
      'image/jpeg',
      0.8
    )
  }
),
cropCancelBtn.addEventListener('click', () => {
  cropClose()
}),
cropCloseBtn.addEventListener('click', () => {
  cropClose()
});
class FsDropdown {
  constructor(e) {
    this.isOpen = !1,
    this.element = document.querySelector(e),
    this.listWrapper = this.element.querySelector('.fs-dropdown__list'),
    this.init()
  }
  init() {
    const e = this.element.querySelector('.fs-dropdown__button');
    e.addEventListener('click', () => {
      this.open()
    });
    const t = this.element.querySelectorAll('.fs-dropdown__list a, .fs-dropdown__list button');
    t.forEach(e => {
      e.addEventListener('click', () => {
        this.close()
      })
    })
  }
  detectClickWithin(e) {
    e.composedPath().includes(this.listWrapper) ||
    this.close()
  }
  open() {
    this.listWrapper.classList.add('fs-dropdown__list--show'),
    setTimeout(
      () => {
        document.addEventListener('click', e => this.detectClickWithin(e), {
          once: !0
        })
      },
      500
    )
  }
  close() {
    this.listWrapper.classList.remove('fs-dropdown__list--show')
  }
}
class FacePlay {
  constructor(e, t) {
    this.xmlns = 'http://www.w3.org/2000/svg',
    this.svgObject = {},
    this.svgElement = {},
    this.landmark = Object.values(e),
    this.delay = 0,
    this.pairs = [
      ['fh_126',
      'le_10'],
      [
        'fh_108',
        'le_10'
      ],
      [
        'fh_108',
        'le_20'
      ],
      [
        'fh_90',
        'le_20'
      ],
      [
        'fh_90',
        'le_31'
      ],
      [
        'fh_72',
        'le_31'
      ],
      [
        'fh_72',
        're_31'
      ],
      [
        'fh_54',
        're_31'
      ],
      [
        'fh_54',
        're_20'
      ],
      [
        'fh_36',
        're_20'
      ],
      [
        'fh_36',
        're_10'
      ],
      [
        'fh_18',
        're_10'
      ],
      [
        'le_31',
        're_31'
      ],
      [
        'nl_62',
        'ul_15'
      ],
      [
        'nl_41',
        'ul_0'
      ],
      [
        'nl_41',
        'ul_7'
      ],
      [
        'nr_41',
        'ul_31'
      ],
      [
        'nr_41',
        'ul_23'
      ],
      [
        'nl_41',
        'ul_15'
      ],
      [
        'nr_41',
        'ul_15'
      ],
      [
        'fcl_31',
        'ul_0'
      ],
      [
        'fcl_15',
        'ul_0'
      ],
      [
        'fcl_15',
        'll_7'
      ],
      [
        'fcl_0',
        'll_7'
      ],
      [
        'fcl_0',
        'll_15'
      ],
      [
        'fcr_31',
        'ul_31'
      ],
      [
        'fcr_15',
        'ul_31'
      ],
      [
        'fcr_15',
        'll_23'
      ],
      [
        'fcl_0',
        'll_23'
      ],
      [
        'fcl_47',
        'ul_0'
      ],
      [
        'fcr_47',
        'ul_31'
      ],
      [
        'nl_41',
        'nm_42'
      ],
      [
        'nr_41',
        'nm_42'
      ],
      [
        'nl_20',
        'nm_42'
      ],
      [
        'nr_20',
        'nm_42'
      ],
      [
        'nl_20',
        'nm_16'
      ],
      [
        'nr_20',
        'nm_16'
      ],
      [
        'le_31',
        'nm_16'
      ],
      [
        're_31',
        'nm_16'
      ],
      [
        'le_31',
        'nl_0'
      ],
      [
        're_31',
        'nr_0'
      ],
      [
        'fcl_63',
        'lfm'
      ],
      [
        'fcl_47',
        'lfm'
      ],
      [
        'lee_31',
        'lfm'
      ],
      [
        'lee_63',
        'lfm'
      ],
      [
        'nl_20',
        'lfm'
      ],
      [
        'ul_0',
        'lfm'
      ],
      [
        'fcr_63',
        'rfm'
      ],
      [
        'fcr_47',
        'rfm'
      ],
      [
        'ree_31',
        'rfm'
      ],
      [
        'ree_63',
        'rfm'
      ],
      [
        'nr_20',
        'rfm'
      ],
      [
        'ul_31',
        'rfm'
      ]
    ],
    this.wrapper = document.querySelector(t),
    this.isSafari = /^((?!chrome|android).)*safari/i.test(navigator.userAgent)
  }
  makeSvgObject(e) {
    var t = Object.values(e);
    let o = [],
    r = [];
    for (let e = 0; e < t.length; e++) {
      this.delay++;
      var i = t[e],
      n = Object.keys(i),
      s = Object.values(i),
      l = this.isSafari ? 250 * e + 'ms' : e;
      for (let e = 0; e < s.length; e++) {
        var a = s[e],
        c = {
          x: a.x,
          y: a.y,
          name: n[e],
          begin: l
        };
        o.push(c);
        var u = s[e + 1];
        u ? (u = {
          x1: a.x,
          y1: a.y,
          x2: u.x,
          y2: u.y,
          begin: l
        }, r.push(u)) : 'fcr_63' !== c.name &&
        'll_7' !== c.name ||
        (c = s[0], c = {
          x1: a.x,
          y1: a.y,
          x2: c.x,
          y2: c.y,
          begin: l
        }, r.push(c))
      }
    }
    this.svgObject.points = o,
    this.svgObject.lines = r,
    this.makeMiddlePoints(),
    this.makeCrossLines(),
    this.makeSvgElement(this.svgObject)
  }
  makeMiddlePoints() {
    var e = this.findPointByName('lee_31'),
    t = this.findPointByName('fcl_47'),
    o = this.findPointByName('ree_31'),
    r = this.findPointByName('fcr_47'),
    i = this.isSafari ? 250 * this.delay + 'ms' : this.delay;
    let n = this.findMiddlePoint(e, t);
    n.name = 'lfm',
    n.begin = i;
    let s = this.findMiddlePoint(o, r);
    s.name = 'rfm',
    s.begin = i,
    this.svgObject.points.push(n, s)
  }
  makeCrossLines() {
    this.pairs.forEach(
      e => {
        var t = this.findPointByName(e[0]),
        o = this.findPointByName(e[1]),
        e = this.isSafari ? 250 * this.delay + 'ms' : this.delay,
        e = {
          x1: t.x,
          y1: t.y,
          x2: o.x,
          y2: o.y,
          begin: e
        };
        this.svgObject.lines.push(e)
      }
    )
  }
  findPointByName(t) {
    let o = {};
    return this.svgObject.points.forEach(e => {
      e.name === t &&
      (o = e)
    }),
    o
  }
  findMiddlePoint(e, t) {
    return {
      x: (e.x + t.x) / 2,
      y: (e.y + t.y) / 2
    }
  }
  makeSvgElement(e) {
    let t = document.createElementNS(this.xmlns, 'svg');
    t.setAttributeNS(null, 'width', '600'),
    t.setAttributeNS(null, 'height', '800'),
    t.setAttributeNS(null, 'viewBox', '0 0 600 800'),
    t.style.width = '100%',
    t.style.height = '100%',
    e.points.forEach(e => {
      t.appendChild(this.makeCircleElement(e))
    }),
    e.lines.forEach(e => {
      t.appendChild(this.makeLineElement(e))
    }),
    this.svgElement = t
  }
  makeCircleElement(e) {
    let t = document.createElementNS(this.xmlns, 'circle');
    return t.setAttributeNS(null, 'cx', e.x),
    t.setAttributeNS(null, 'cy', e.y),
    t.setAttributeNS(null, 'r', 3),
    this.isSafari ? (
      t.setAttributeNS(null, 'name', e.name),
      t.setAttributeNS(null, 'fill', '#ffffffdd'),
      t.appendChild(this.makeAnimateElement('r', 3, e.begin))
    ) : t.setAttributeNS(null, 'class', 'circle start-' + e.begin),
    t
  }
  makeLineElement(e) {
    let t = document.createElementNS(this.xmlns, 'line');
    return t.setAttributeNS(null, 'x1', e.x1),
    t.setAttributeNS(null, 'y1', e.y1),
    this.isSafari ? (
      t.setAttributeNS(null, 'x2', e.x1),
      t.setAttributeNS(null, 'y2', e.y1),
      t.setAttributeNS(null, 'stroke', '#ffffffdd'),
      t.setAttributeNS(null, 'stroke-width', '0.5px'),
      t.appendChild(this.makeAnimateElement('x2', e.x2, e.begin)),
      t.appendChild(this.makeAnimateElement('y2', e.y2, e.begin))
    ) : (
      t.setAttributeNS(null, 'x2', e.x2),
      t.setAttributeNS(null, 'y2', e.y2),
      t.setAttributeNS(null, 'class', 'line start-' + e.begin)
    ),
    t
  }
  makeAnimateElement(e, t, o) {
    let r = document.createElementNS(this.xmlns, 'animate');
    return r.setAttributeNS(null, 'begin', o),
    r.setAttributeNS(null, 'fill', 'freeze'),
    r.setAttributeNS(null, 'dur', '250ms'),
    r.setAttributeNS(null, 'attributeName', e),
    r.setAttributeNS(null, 'to', t),
    r
  }
  reset() {
    this.delay = 0,
    this.wrapper.innerHTML = ''
  }
  play() {
    this.reset(),
    this.makeSvgObject(this.landmark),
    this.svgElement.addEventListener('click', () => {
      this.svgElement.classList.toggle('invisible')
    }),
    this.wrapper.appendChild(this.svgElement)
  }
}
YES.live(
  document,
  'click',
  '.btn-learn-more',
  function (e) {
    e.preventDefault(),
    $(this).jajax({
      data: {
        target: 'face-recognition-card-container'
      }
    }).then(() => {
      navAutoScroll('#result')
    })
  }
),
YES.live(
  document,
  'click',
  '.back-to-face-btn',
  e => {
    e.preventDefault(),
    $(e.target.closest('.back-to-face-btn')).jajax()
  }
),
YES.live(
  document,
  'click',
  '#result .quiz-opener',
  function (e) {
    resultPopup.close(),
    faceRecognition.openQuiz()
  }
),
YES.live(
  document,
  'click',
  '#result .recognition-again-opener',
  function (e) {
    resultPopup.close(),
    faceRecognition.reUpload()
  }
);
const RecognitionDropdown = new FsDropdown('.recognition-dropdown');
function navAutoScroll(e) {
  e = e ||
  window.location.hash;
  const t = document.querySelector(e + ' .fd-nav'),
  o = document.querySelectorAll(e + ' .fd-nav li');
  o.forEach(
    e => {
      e.classList.contains('active') &&
      (
        e = e.offsetLeft + e.offsetWidth / 2 - t.offsetWidth / 2,
        t.scrollTo({
          top: 0,
          left: e,
          behavior: 'smooth'
        })
      )
    }
  )
}
YES.live(
  document,
  'click',
  '.privacy-clear-info',
  e => {
    $.jmodal.confirm(
      e__('Are you sure you want to delete the face record?'),
      () => {
        var e = document.querySelector('.privacy-clear-info'),
        t = e.dataset.requestUrl;
        $(e).jajax({
          url: t,
          type: 'get'
        }).then(function (e) {
          'ok' === e.status &&
          faceRecognition.reset()
        })
      },
      function () {
      },
      e__('Yes'),
      e__('No')
    )
  }
),
faceRecognition = {
  init() {
    this.uploadTrigger = document.querySelectorAll('.fs-recognition__upload-trigger'),
    this.cancelTrigger = document.querySelectorAll('.fs-recognition__top__close'),
    this.reUploadTrigger = document.querySelectorAll('.recognition-again-opener'),
    this.inputElement = document.querySelector('#upload_input'),
    this.sectionWrapper = document.querySelector('.fs-section--recognition'),
    this.emptyWrapper = document.querySelector('.fs-recognition--empty'),
    this.loadedWrapper = document.querySelector('.fs-recognition--loaded'),
    this.pendingWrapper = document.querySelector('.fs-recognition__bottom__pending'),
    this.resaultWrapper = document.querySelector('.fs-recognition__bottom__resault'),
    this.progressElement = document.querySelector('.fs-recognition__progress__value'),
    this.progressTimer = '',
    this.quizTrigger = document.querySelectorAll('.quiz-opener'),
    this.quizWrapper = document.querySelector('.fs-recognition--quiz'),
    this.recognitionBtn = document.querySelector('.fs-section--recognition .recognition-opener'),
    this.quizBtn = document.querySelector('.fs-section--recognition .quiz-opener'),
    this.hasRecognition = !1,
    this.uploadTrigger.forEach(
      e => {
        e.addEventListener('click', () => {
          checkLogin(() => this.inputElement.click())
        })
      }
    ),
    this.inputElement &&
    this.inputElement.addEventListener(
      'change',
      () => {
        cropImage(this.inputElement.files, finishRecognitionCrop)
      }
    ),
    this.reUploadTrigger.forEach(
      e => {
        e.addEventListener('click', () => {
          checkLogin(() => this.reUpload())
        })
      }
    ),
    this.cancelTrigger.forEach(e => {
      e.addEventListener('click', () => {
        this.cancel()
      })
    }),
    this.quizTrigger.forEach(e => {
      e.addEventListener('click', () => {
        this.openQuiz()
      })
    }),
    this.recognitionBtn &&
    this.recognitionBtn.addEventListener('click', () => {
      this.closeQuiz()
    })
  },
  reset() {
    this.emptyWrapper.hidden = !1,
    this.loadedWrapper.hidden = !0
  },
  loading() {
    this.loadedWrapper.hidden = !1,
    this.emptyWrapper.hidden = !0,
    this.showPending()
  },
  showPending() {
    this.pendingWrapper.hidden = !1,
    this.resaultWrapper.hidden = !0,
    this.loadedWrapper.classList.remove('fs-recognition--extended'),
    this.progressElement.style.transition = '5s',
    this.progressTimer = setTimeout(() => {
      this.progressElement.style.width = '90%'
    }, 10)
  },
  formatLandMark(e) {
    return [{
      fh_18: e.fh_18,
      fh_36: e.fh_36,
      fh_54: e.fh_54,
      fh_72: e.fh_72,
      fh_90: e.fh_90,
      fh_108: e.fh_108,
      fh_126: e.fh_126,
      fcl_63: e.fcl_63,
      fcl_47: e.fcl_47,
      fcl_31: e.fcl_31,
      fcl_15: e.fcl_15,
      fcl_0: e.fcl_0,
      fcr_15: e.fcr_15,
      fcr_31: e.fcr_31,
      fcr_47: e.fcr_47,
      fcr_63: e.fcr_63
    },
    {
      le_0: e.le_0,
      le_10: e.le_10,
      le_20: e.le_20,
      le_31: e.le_31
    },
    {
      re_0: e.re_0,
      re_10: e.re_10,
      re_20: e.re_20,
      re_31: e.re_31
    },
    {
      lee_31: e.lee_31,
      lee_47: e.lee_47,
      lee_63: e.lee_63
    },
    {
      ree_31: e.ree_31,
      ree_47: e.ree_47,
      ree_63: e.ree_63
    },
    {
      nl_0: e.nl_0,
      nl_20: e.nl_20,
      nl_41: e.nl_41,
      nl_62: e.nl_62,
      nr_41: e.nr_41,
      nr_20: e.nr_20,
      nr_0: e.nr_0
    },
    {
      ul_0: e.ul_0,
      ul_7: e.ul_7,
      ul_15: e.ul_15,
      ul_23: e.ul_23,
      ul_31: e.ul_31,
      ll_23: e.ll_23,
      ll_15: e.ll_15,
      ll_7: e.ll_7
    },
    {
      nm_16: e.nm_16,
      nm_42: e.nm_42
    }
    ]
  },
  showResult(e) {
    clearTimeout(this.progressTimer),
    this.progressElement.style.width = '100%';
    var t = this.formatLandMark(e.faceMeta);
    const o = new FacePlay(t, '.face-shape-scaner');
    o.play(),
    document.querySelector('#recommended-glasses').href = e.recommendUrl,
    window.gtag &&
    gtag(
      'event',
      'conversion',
      {
        send_to: 'AW-665645844/EWKGCLSqjo8YEJTms70C'
      }
    ),
    setTimeout(
      () => {
        Hash.set('#result'),
        document.querySelector('#result .quiz-opener').addEventListener('click', () => {
          faceRecognition.openQuiz()
        }),
        document.querySelector('#result .recognition-again-opener').addEventListener('click', () => {
          checkLogin(() => faceRecognition.reUpload())
        }),
        this.loadedWrapper.classList.add('fs-recognition--extended'),
        this.pendingWrapper.hidden = !0,
        this.resaultWrapper.hidden = !1
      },
      3000
    )
  },
  openQuiz() {
    !1 === this.loadedWrapper.hidden ? (this.hasRecognition = !0, this.loadedWrapper.hidden = !0) : this.emptyWrapper.hidden = !0,
    this.quizBtn.hidden = !0,
    this.recognitionBtn.hidden = !1,
    this.quizWrapper.hidden = !1
  },
  closeQuiz() {
    fsQuiz.reset(),
    this.quizBtn.hidden = !1,
    this.recognitionBtn.hidden = !0,
    this.quizWrapper.hidden = !0,
    this.hasRecognition ? this.loadedWrapper.hidden = !1 : this.emptyWrapper.hidden = !1
  },
  cancel() {
    this.inputElement.value = '',
    this.progressElement.style.width = '5%',
    document.querySelector('.face-shape-scaner').innerHTML = '',
    this.closeQuiz(),
    this.reset()
  },
  reUpload() {
    this.inputElement.click()
  }
},
faceRecognition.init(),
recognitionUploader = {
  upload(e) {
    var t = document.querySelector('.crop-continue').dataset.postUrl;
    let o = new FormData;
    o.append('face', e),
    $.jajax({
      url: t,
      type: 'post',
      data: o,
      cache: !1,
      processData: !1,
      contentType: !1
    }).then(
      function (t) {
        if ('ok' === t.status) {
          faceRecognition.showResult(t.opts),
          document.querySelector('.upload_file_path_key').setAttribute('value', t.opts.uploadImageKey),
          document.querySelector('.upload_recognition_id').setAttribute('value', t.opts.recognitionId),
          document.querySelector('.fs-recognition__bottom .fs-btn--white').setAttribute('href', t.opts.recommendUrl);
          let e = document.createElement('img');
          e.src = t.opts.realImagePath,
          pollImgUploader.makeThumb(e, !1)
        }
      }
    )
  }
};
class FaceShapeQuiz {
  constructor() {
    this.faceQuizTable = {
      hairline: {
        rounded: [
          'round',
          'oval',
          'heart',
          'oblong'
        ],
        straight: [
          'sqaure',
          'rectangle',
          'heart',
          'triangle',
          'oblong'
        ],
        narrow: [
          'oval',
          'pear',
          'diamond',
          'oblong'
        ]
      },
      jawline: {
        round: [
          'round',
          'oval',
          'oblong'
        ],
        square: [
          'square',
          'rectangle',
          'pear',
          'oblong'
        ],
        pointy: [
          'oval',
          'heart',
          'triangle',
          'diamond',
          'oblong'
        ]
      },
      f2c: {
        wider: [
          'triangle',
          'pear',
          'oblong'
        ],
        equal: [
          'round',
          'oval',
          'square',
          'rectangle',
          'heart',
          'triangle',
          'pear',
          'oblong'
        ],
        narrower: [
          'round',
          'oval',
          'heart',
          'pear',
          'diamond',
          'oblong'
        ]
      },
      j2c: {
        wider: [
          'pear',
          'oblong'
        ],
        equal: [
          'round',
          'oval',
          'square',
          'rectangle',
          'oblong'
        ],
        narrower: [
          'round',
          'oval',
          'heart',
          'triangle',
          'diamond',
          'oblong'
        ]
      },
      longface: {
        yes: [
          'oval',
          'rectangle',
          'heart',
          'triangle',
          'pear',
          'diamond',
          'oblong'
        ],
        no: [
          'round',
          'square',
          'heart',
          'triangle',
          'pear',
          'diamond'
        ]
      }
    }
  }
  setOption(e, t) {
    const o = this.faceQuizTable[e][t];
    if (void 0 === this.results) return this.results = o,
    this;
    t = this.results.filter(e => o.includes(e));
    return t.length &&
    (this.results = t),
    this
  }
  getResult() {
    return this.results[0]
  }
}
function initCounter() {
  const e = document.querySelectorAll('.ask-popup__form__textarea');
  function r(e, t) {
    var o = Number(e.getAttribute('maxlength')),
    e = e.value.length;
    t.innerHTML = ''.concat(e, ' / ').concat(o),
    t.classList.toggle('max', o === e)
  }
  e.forEach(
    e => {
      const t = e.querySelector('textarea'),
      o = e.querySelector('.ask-popup__form__textarea__count');
      r(t, o),
      t.addEventListener('input', () => r(t, o))
    }
  )
}
function reRenderFace() {
  var e;
  document.querySelector('#crop_result').innerHTML.trim() &&
  (
    e = document.querySelector('#crop_result img'),
    $.jajax({
      url: e.dataset.requestUrl,
      type: 'get'
    }).then(
      e => {
        if ('ok' === e.status) {
          e = faceRecognition.formatLandMark(e.opts);
          const t = new FacePlay(e, '.face-shape-scaner');
          t.play()
        }
      }
    )
  )
}
fsQuiz = {
  init() {
    this.conatiner = document.querySelector('.fs-recognition--quiz'),
    this.wrapper = document.querySelector('.fs-recognition--quiz form'),
    this.items = document.querySelectorAll('.fs-recognition--quiz .quiz-section'),
    this.resault = document.querySelector('.fs-recognition--quiz .quiz-resault'),
    this.resaultTitle = document.querySelector(
      '.fs-recognition--quiz .quiz-resault #quiz-resault__resault__title'
    ),
    this.resaultImgs = document.querySelectorAll('.fs-recognition--quiz .quiz-resault__image-wrapper img'),
    this.resetTrigger = document.querySelectorAll('.fs-recognition--quiz .quiz-reset'),
    this.quizResShowBtn = document.querySelector('.quiz-resault__bottom__resault .fs-btn-open'),
    this.quizResShowRecommendedBtn = document.querySelector('.quiz-resault__bottom__resault .fs-btn-open-recommended'),
    this.wrapper.scrollTo(0, 0),
    this.items.forEach(
      (e, t) => {
        e.querySelectorAll('.fs-btn').forEach(e => {
          e.addEventListener('click', () => {
            this.goNext(t)
          })
        })
      }
    ),
    this.resetTrigger.forEach(e => {
      e.addEventListener('click', () => {
        this.reset()
      })
    }),
    this.quizResShowBtn.addEventListener(
      'click',
      function () {
        var e = document.querySelector('.quiz-resault__bottom__resault .fs-btn-open').dataset.currentFace,
        t = document.querySelector('.quiz-resault__bottom__resault .fs-btn-open').dataset.faceShape,
        e = JSON.parse(t) [e];
        Hash.set('#guide').setAttribute('href', e)
      }
    )
  },
  goNext(e) {
    var t = this.wrapper.offsetWidth;
    e < this.items.length - 1 ? this.wrapper.scrollBy({
      top: 0,
      left: t,
      behavior: 'smooth'
    }) : setTimeout(() => {
      this.goResault()
    }, 100)
  },
  goResault() {
    var t = $('#quiz_form').serializeArray();
    const o = new FaceShapeQuiz;
    for (let e = 0; e < t.length; e++) {
      var r = t[e];
      o.setOption(r.name, r.value)
    }
    var e = o.getResult();
    this.showResult(
      e,
      [
        'round',
        'oval',
        'square',
        'rectangle',
        'heart',
        'triangle',
        'pear',
        'diamond',
        'oblong'
      ].indexOf(e)
    )
  },
  showResult(e, o) {
    this.wrapper.hidden = !0,
    this.resault.hidden = !1;
    let t = 'oval' !== e &&
    'oblong' !== e ? 'a ' : 'an ';
    this.resaultTitle.innerHTML = t + '"' + e + '"',
    document.querySelector('.quiz-resault__bottom__resault .fs-btn-open').dataset.currentFace = e.toLowerCase(),
    this.conatiner.classList.add('fs-recognition--quiz--extended'),
    this.resaultImgs.forEach((e, t) => {
      e.classList.toggle('show', t === o)
    });
    var r = JSON.parse(this.quizResShowRecommendedBtn.dataset.faceShapeRecommend);
    this.quizResShowRecommendedBtn.setAttribute('href', r[e.toLowerCase()])
  },
  reset() {
    document.querySelector('#quiz_form').reset(),
    this.conatiner.classList.remove('fs-recognition--quiz--extended'),
    this.wrapper.hidden = !1,
    this.wrapper.scrollTo(0, 0),
    this.resault.hidden = !0
  }
},
fsQuiz.init(),
faceShapeNav = {
  faceShapeList: document.querySelector('.fs-carousel--guide ul'),
  faceShapeListItems: document.querySelectorAll('.fs-carousel--guide ul li'),
  faceShapeNav: document.querySelector('.fs-nav__indicator'),
  faceShapeNavItems: document.querySelectorAll('.fs-nav ul li'),
  faceShapeNavAnimations: document.querySelectorAll('.fs-nav__indicator animate'),
  arrowLeft: document.querySelector('.fs-carousel--guide .fs-carousel__arrow--left'),
  arrowRight: document.querySelector('.fs-carousel--guide .fs-carousel__arrow--right'),
  currentIndex: 0,
  isAutoScrolling: !1,
  scrollTimer: null,
  windowLoaded: !1,
  init() {
    this.arrowClickListener(),
    this.scrollListener(),
    this.displayArrow(),
    this.setActiveNavItem(0),
    window.addEventListener(
      'load',
      () => {
        this.windowLoaded = !0,
        document.querySelector('.fs-nav').classList.add('fs-nav--ready'),
        this.faceShapeNavAnimations[this.currentIndex].beginElement()
      }
    )
  },
  scrollToIndex(e) {
    e < 0 ? e = 0 : e >= this.faceShapeListItems.length &&
    (e = this.faceShapeListItems.length - 1),
    this.currentIndex = e,
    this.setActiveNavItem(e),
    this.isAutoScrolling = !0;
    var t = this.faceShapeList.offsetWidth * e;
    this.faceShapeList.scrollTo({
      top: 0,
      left: t,
      behavior: 'smooth'
    });
    t = 100 / this.faceShapeNavItems.length;
    this.faceShapeNav.style.left = e * t + '%',
    this.faceShapeNavAnimations[e] &&
    this.windowLoaded &&
    this.faceShapeNavAnimations[e].beginElement(),
    setTimeout(() => {
      this.isAutoScrolling = !1
    }, 1000)
  },
  setActiveNavItem(o) {
    this.faceShapeNavItems.forEach((e, t) => {
      e.classList.toggle('active', t === o)
    })
  },
  arrowClickListener() {
    this.arrowLeft.addEventListener('click', () => {
      this.scrollToIndex(this.currentIndex - 1)
    }),
    this.arrowRight.addEventListener('click', () => {
      this.scrollToIndex(this.currentIndex + 1)
    })
  },
  scrollListener() {
    this.faceShapeList.addEventListener(
      'scroll',
      () => {
        this.displayArrow(),
        clearTimeout(this.scrollTimer),
        this.scrollTimer = setTimeout(
          () => {
            var e;
            this.isAutoScrolling ||
            (
              e = Math.round(this.faceShapeList.scrollLeft / this.faceShapeList.offsetWidth),
              this.scrollToIndex(e)
            )
          },
          250
        )
      }
    )
  },
  displayArrow() {
    var e = this.faceShapeList.scrollLeft;
    e < 10 ? (
      this.current = 0,
      this.arrowLeft.classList.add('fs-carousel__arrow--disabled'),
      this.arrowRight.classList.remove('fs-carousel__arrow--disabled')
    ) : e + this.faceShapeList.offsetWidth > this.faceShapeList.scrollWidth - 10 ? (
      this.current = 8,
      this.arrowLeft.classList.remove('fs-carousel__arrow--disabled'),
      this.arrowRight.classList.add('fs-carousel__arrow--disabled')
    ) : (
      this.current++,
      this.arrowLeft.classList.remove('fs-carousel__arrow--disabled'),
      this.arrowRight.classList.remove('fs-carousel__arrow--disabled')
    )
  }
},
faceShapeNav.init(),
pollImgUploader = {
  form: document.querySelector('.ask-popup__form'),
  uploadInput: document.querySelector('.ask-popup__form .ask-popup__form__upload__input'),
  uploadBox: document.querySelector('.ask-popup__form .ask-popup__form__upload'),
  uploadThumbWrapper: document.querySelector('.ask-popup__form .ask-popup__form__upload__pending'),
  uploadTrigger: document.querySelectorAll(
    '.ask-popup__form .ask-popup__form__upload__icon, .ask-popup__form .ask-popup__form__upload__text button'
  ),
  submitBtn: document.querySelector('.ask-popup__form .fs-btn--primary'),
  init() {
    this.uploadTrigger.forEach(
      e => {
        e.addEventListener('click', () => {
          this.uploadInput.click()
        })
      }
    ),
    this.submitBtn.addEventListener(
      'click',
      e => {
        e.preventDefault(),
        this.valdation() &&
        this.submitForm()
      }
    ),
    this.deValdation(),
    this.uploadInput.addEventListener(
      'change',
      () => {
        cropImage(this.uploadInput.files, finishAskCrop)
      }
    );
    const t = this.form.querySelector('.ask-popup__form__upload');
    t.addEventListener('drag', e => {
      e.preventDefault(),
      e.stopPropagation()
    }),
    t.addEventListener('dragstart', e => {
      e.preventDefault(),
      e.stopPropagation()
    }),
    t.addEventListener('dragend', e => {
      e.preventDefault(),
      e.stopPropagation()
    }),
    t.addEventListener(
      'dragover',
      e => {
        e.preventDefault(),
        e.stopPropagation(),
        t.classList.add('is-dragover')
      }
    ),
    t.addEventListener(
      'dragenter',
      e => {
        e.preventDefault(),
        e.stopPropagation(),
        t.classList.add('is-dragover')
      }
    ),
    t.addEventListener(
      'dragleave',
      e => {
        e.preventDefault(),
        e.stopPropagation(),
        t.classList.remove('is-dragover')
      }
    ),
    t.addEventListener(
      'dragend',
      e => {
        e.preventDefault(),
        e.stopPropagation(),
        t.classList.remove('is-dragover')
      }
    ),
    t.addEventListener(
      'drop',
      e => {
        e.preventDefault(),
        e.stopPropagation(),
        t.classList.remove('is-dragover'),
        cropImage(e.dataTransfer.files, finishAskCrop)
      }
    )
  },
  submitForm() {
    return $(this.form).jajax().then(
      e => {
        'ok' === e.status &&
        (
          document.querySelector('.poll-grid'),
          $('.poll-grid').prepend(e.opts.html),
          document.querySelector('.btn-my-post').classList.remove('none'),
          Hash.close(),
          document.querySelector('.ask-popup__form').reset(),
          pollImgUploader.resetImg()
        )
      }
    ),
    !1
  },
  upload(e, t) {
    let o = new FormData;
    o.append('file', e),
    $.jajax({
      type: 'post',
      url: this.uploadInput.dataset.requestUrl,
      processData: !1,
      cache: !1,
      contentType: !1,
      data: o
    }).then(
      e => {
        'ok' === e.status &&
        (
          document.querySelector('.upload_file_path_key').setAttribute('value', e.opts.uploadImageKey),
          document.querySelector('.upload_recognition_id').setAttribute('value', ''),
          t()
        )
      }
    )
  },
  valdation() {
    const e = this.form.querySelectorAll('.ask-popup__form__item');
    let r = !0;
    return e.forEach(
      e => {
        var t = e.querySelector('input'),
        o = e.querySelector('.ask-popup__form__upload__thumb');
        !t ||
        t.value ||
        o ? e.classList.remove('ask-popup__form__item--warn') : (e.classList.add('ask-popup__form__item--warn'), r = !1)
      }
    ),
    r
  },
  deValdation() {
    const e = this.form.querySelectorAll('.ask-popup__form__item');
    e.forEach(
      e => {
        const t = e.querySelector('input');
        t &&
        t.addEventListener(
          'input',
          () => {
            e.classList.remove('ask-popup__form__item--warn')
          }
        )
      }
    )
  },
  makeThumb(e, t) {
    this.resetImg();
    let o = document.createElement('div');
    o.classList.add('ask-popup__form__upload__thumb', 'pending');
    let r = document.createElement('div');
    r.classList.add('close'),
    r.innerHTML = '<svg width="24" height="24" viewbox="0 0 24 24"><path d="M12 11.293l10.293-10.293.707.707-10.293 10.293 10.293 10.293-.707.707-10.293-10.293-10.293 10.293-.707-.707 10.293-10.293-10.293-10.293.707-.707 10.293 10.293z"/></svg>',
    r.addEventListener(
      'click',
      () => {
        this.resetImg(),
        !1 !== t &&
        $.jajax({
          url: this.uploadInput.dataset.removeUrl,
          type: 'get',
          data: {
            file_path_key: document.querySelector('.upload_file_path_key').value
          }
        }).then(
          () => {
            document.querySelector('.upload_file_path_key').setAttribute('value', '')
          }
        )
      }
    ),
    e.classList.add('pending'),
    o.appendChild(e),
    o.appendChild(r),
    this.uploadThumbWrapper.append(o),
    this.uploadBox.classList.remove('empty'),
    this.submitBtn.classList.remove('fs-btn--disabled')
  },
  resetImg() {
    this.uploadBox.classList.add('empty'),
    this.submitBtn.classList.add('fs-btn--disabled'),
    this.uploadThumbWrapper.innerHTML = '',
    this.uploadInput.value = ''
  }
},
pollImgUploader.init(),
initCounter(),
YES.live(
  document,
  'click',
  '.face-shape-item-a',
  e => {
    e.preventDefault();
    let t = e.target.closest('.face-shape-item-a'),
    o = e.target.closest('ul');
    o.querySelectorAll('li').forEach(e => {
      e.classList.remove('active')
    }),
    t.closest('li').classList.add('active'),
    navAutoScroll(),
    $(t).jajax({
      data: {
        pjax: 1,
        target: t.closest('hash-popup').id
      }
    })
  }
),
YES.live(
  document,
  'click',
  '.next-btn',
  e => {
    e.preventDefault();
    const t = document.querySelector('#poll');
    t.dataset.loading = 'true',
    $(e.target).jajax({
      triggerEvents: !1
    }).done(
      e => {
        'ok' === e.status &&
        document.querySelector('#poll-item-container .poll-dropdown') &&
        new FsDropdown('#poll-item-container .poll-dropdown')
      }
    ).always(() => delete t.dataset.loading)
  }
),
YES.live(
  document,
  'click',
  '.face-shape-item-li',
  e => {
    let o;
    o = e.target.classList.contains('face-shape-item-li') ? $(e.target) : $(e.target).closest('.face-shape-item-li'),
    o.parent('ul').children('li').each(
      function (e, t) {
        t.classList.contains('active') &&
        t.classList.remove('active')
      }
    ),
    o.addClass('active');
    var t = window.devicePixelRatio,
    r = window.screen.width,
    i = window.screen.height,
    n = o.data('shape-id'),
    e = document.querySelector('#poll-popup__chart__container');
    let s = e.dataset.pollId;
    e = e.dataset.requestUrl;
    $.jajax({
      url: e,
      type: 'post',
      data: {
        shape_id: n,
        poll_id: s,
        w: r,
        h: i,
        dppx: t
      }
    }).then(
      function (t) {
        if ('ok' === t.status) {
          let e = document.querySelector('.poll-grid__item-' + s);
          o.parent('ul').html(t.opts.html),
          e &&
          !e.classList.contains('fs-btn--primary') &&
          $(e).find('.poll-popup-opener').addClass('fs-btn--primary')
        }
      }
    )
  }
),
reRenderFace(),
YES.live(
  document,
  'click',
  '.dont-agree-btn',
  function () {
    document.querySelector('#result .recognition-popup__section--btns').scrollIntoView({
      behavior: 'smooth'
    })
  }
),
YES.live(
  document,
  'click',
  '.btn-show-more',
  function (e) {
    e.preventDefault();
    let t = e.target;
    $(t).jajax({
      data: {
        page: t.dataset.nextPage
      }
    }).then(
      e => {
        'ok' === e.status &&
        (
          $(document.querySelector('.poll-grid')).append(e.opts.html),
          e.opts.nextPage ? t.dataset.nextPage = e.opts.nextPage : t.classList.add('none')
        )
      }
    )
  }
);
let pollScrollTimer,
canLoad = !0;
$('.poll-grid').on(
  'scroll',
  () => {
    var e = $('.poll-grid').scrollLeft(),
    t = $('.poll-grid').get(0).scrollWidth - $('.poll-grid').innerWidth();
    let o = document.querySelector('.btn-show-more');
    $(window).innerWidth() < 672 &&
    t <= e + 16 &&
    canLoad &&
    (
      o &&
      !o.classList.contains('none') &&
      o.click(),
      canLoad = !1,
      clearTimeout(pollScrollTimer),
      pollScrollTimer = setTimeout(() => {
        canLoad = !0
      }, 1000)
    )
  }
);
let initImageUrl = document.querySelector('.upload_file_path_key').dataset.httpUrl.trim();
if (initImageUrl) {
  let e = document.createElement('img');
  e.src = initImageUrl,
  pollImgUploader.makeThumb(e, !1)
}
function modifyValdation() {
  const e = document.querySelectorAll('.ask-popup__form__modify .ask-popup__form__item');
  let r = !0;
  return e.forEach(
    e => {
      var t = e.querySelector('input'),
      o = e.querySelector('.ask-popup__form__upload__thumb');
      !t ||
      t.value ||
      o ? e.classList.remove('ask-popup__form__item--warn') : (e.classList.add('ask-popup__form__item--warn'), r = !1)
    }
  ),
  r
}
function initModifyValdation() {
  const e = document.querySelectorAll('.ask-popup__form__modify .ask-popup__form__item');
  e.forEach(
    e => {
      const t = e.querySelector('input');
      t &&
      t.addEventListener(
        'input',
        () => {
          e.classList.remove('ask-popup__form__item--warn')
        }
      )
    }
  )
}
YES.live(
  document,
  'click',
  '.delete-this-post',
  function (e) {
    e.preventDefault();
    let t = this;
    $.jmodal.confirm(
      e__('Are you sure you want to delete this poll?'),
      function () {
        $(t).jajax().then(
          () => {
            Hash.close();
            var e = t.dataset.pollId;
            document.querySelector('.poll-grid__item-' + e).remove()
          }
        )
      }
    )
  }
),
YES.live(
  document,
  'submit',
  '.ask-popup__form__modify',
  function (e) {
    return e.preventDefault(),
    modifyValdation() &&
    $(this).jajax().then(e => {
      'ok' === e.status &&
      Hash.close()
    }),
    !1
  }
),
YES.live(
  document,
  'click',
  '.modify-this-post',
  function (t) {
    t.preventDefault(),
    $(this).jajax().then(
      e => {
        'ok' === e.status &&
        (
          $(t.target).closest('.poll-container').html(e.opts.html),
          initCounter(),
          initModifyValdation()
        )
      }
    )
  }
),
!0 === initOpenQuiz &&
faceRecognition.openQuiz(),
document.querySelector('#download-btn').addEventListener(
  'click',
  e => {
    e.preventDefault();
    e = e.currentTarget.href;
    let t = window.open(
      e,
      __('All Face Shape Recommendations'),
      'resizable,scrollbars,location=no,status=no'
    );
    t.document.write(
      '\n        <html>\n            <head>\n            <title>All Face Shape Recommendations</title>\n            <style>body {text-align: center;} img {width: 100%; max-width: 1024px; min-width: 320px;}</style>\n            </head>\n            <body><img src="'.concat(e, '"></body>\n        </html>\n    ')
    ),
    t.history.pushState(null, '', window.location.href)
  }
),
Array.from(document.querySelectorAll('.faq-details h3')).forEach(
  function (e) {
    e.addEventListener(
      'click',
      function () {
        YES.upDown(
          this.nextElementSibling,
          function () {
            this.parentElement.classList.toggle('open')
          },
          null,
          null,
          '.faq-details .desc.open .box'
        )
      }
    )
  }
),
YES.live(
  document,
  'click',
  '.fd-section--toolbar a',
  function (e) {
    e.preventDefault();
    var t = e.target.href,
    e = document.querySelector('.fd-section--recommended input').checked ? '' : '?sunglasses';
    window.open(t + e)
  }
);
//# sourceMappingURL=sourcemaps/face--recognition--index.js.map
