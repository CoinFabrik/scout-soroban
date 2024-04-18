"use strict";(self.webpackChunkscout=self.webpackChunkscout||[]).push([[9735],{9613:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>b});var n=r(9496);function o(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function a(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?a(Object(r),!0).forEach((function(t){o(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):a(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,o=function(e,t){if(null==e)return{};var r,n,o={},a=Object.keys(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||(o[r]=e[r]);return o}(e,t);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);for(n=0;n<a.length;n++)r=a[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(o[r]=e[r])}return o}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},p=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},u="mdxType",d={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,o=e.mdxType,a=e.originalType,l=e.parentName,p=s(e,["components","mdxType","originalType","parentName"]),u=c(r),m=o,b=u["".concat(l,".").concat(m)]||u[m]||d[m]||a;return r?n.createElement(b,i(i({ref:t},p),{},{components:r})):n.createElement(b,i({ref:t},p))}));function b(e,t){var r=arguments,o=t&&t.mdxType;if("string"==typeof e||o){var a=r.length,i=new Array(a);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[u]="string"==typeof e?e:o,i[1]=s;for(var c=2;c<a;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},9921:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>d,frontMatter:()=>a,metadata:()=>s,toc:()=>c});var n=r(2564),o=(r(9496),r(9613));const a={sidebar_position:4},i="Contribute",s={unversionedId:"contribute",id:"contribute",title:"Contribute",description:"Thank you for your interest in contributing to the development of new detectors.",source:"@site/docs/contribute.md",sourceDirName:".",slug:"/contribute",permalink:"/scout-soroban/docs/contribute",draft:!1,editUrl:"https://github.com/CoinFabrik/scout-soroban/docs/contribute.md",tags:[],version:"current",sidebarPosition:4,frontMatter:{sidebar_position:4},sidebar:"docsSidebar",previous:{title:"Assert  violation",permalink:"/scout-soroban/docs/detectors/assert-violation"},next:{title:"Architecture",permalink:"/scout-soroban/docs/architecture"}},l={},c=[{value:"Getting Started",id:"getting-started",level:3},{value:"Detectors",id:"detectors",level:3},{value:"Test Cases",id:"test-cases",level:3}],p={toc:c},u="wrapper";function d(e){let{components:t,...r}=e;return(0,o.kt)(u,(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,o.kt)("h1",{id:"contribute"},"Contribute"),(0,o.kt)("p",null,"Thank you for your interest in contributing to the development of new detectors."),(0,o.kt)("h3",{id:"getting-started"},"Getting Started"),(0,o.kt)("p",null,"Create a new issue on our ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban"},"repository")," with the name of the new detector or test case you wish to contribute. Then, link a new branch to that issue."),(0,o.kt)("p",null,"If your detector or test case doesn't belong to an existing ",(0,o.kt)("a",{parentName:"p",href:"https://coinfabrik.github.io/scout-soroban/docs/vulnerabilities#vulnerability-classes"},"vulnerability class"),", please provide documentation for the new vulnerability class you're proposing."),(0,o.kt)("blockquote",null,(0,o.kt)("p",{parentName:"blockquote"},"\u2757 ",(0,o.kt)("strong",{parentName:"p"},"Requirement"),": All detectors and test cases should follow the ",(0,o.kt)("strong",{parentName:"p"},"kebab-case")," naming convention, using ",(0,o.kt)("strong",{parentName:"p"},"lowercase and hyphens")," only.")),(0,o.kt)("h3",{id:"detectors"},"Detectors"),(0,o.kt)("p",null,"To contribute a new detector:"),(0,o.kt)("ol",null,(0,o.kt)("li",{parentName:"ol"},(0,o.kt)("p",{parentName:"li"},"Choose an appropriate template. Browse our templates at ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/templates/detectors"},(0,o.kt)("inlineCode",{parentName:"a"},"templates/detectors")),". Decide on the ",(0,o.kt)("inlineCode",{parentName:"p"},"early-lint")," or ",(0,o.kt)("inlineCode",{parentName:"p"},"late-lint")," template, based on whether you want to lint before or after macro expansion.")),(0,o.kt)("li",{parentName:"ol"},(0,o.kt)("p",{parentName:"li"},"Add your modified detector files to a new folder, naming it after your detector, inside the ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/detectors"},(0,o.kt)("inlineCode",{parentName:"a"},"detectors"))," directory."))),(0,o.kt)("h3",{id:"test-cases"},"Test Cases"),(0,o.kt)("p",null,"To contribute a new test case:"),(0,o.kt)("ol",null,(0,o.kt)("li",{parentName:"ol"},(0,o.kt)("p",{parentName:"li"},"Determine the ",(0,o.kt)("a",{parentName:"p",href:"https://coinfabrik.github.io/scout-soroban/docs/vulnerabilities#vulnerability-classes"},"vulnerability class")," to which your test case belongs. Then, create a new sub-folder under that class in the ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/test-cases"},(0,o.kt)("inlineCode",{parentName:"a"},"test-cases"))," directory. Remember to append the detector number at the end, separated by a hyphen.")),(0,o.kt)("li",{parentName:"ol"},(0,o.kt)("p",{parentName:"li"},"Within this sub-folder, create two directories: ",(0,o.kt)("inlineCode",{parentName:"p"},"vulnerable-example")," and ",(0,o.kt)("inlineCode",{parentName:"p"},"remediated-example"),". Fill each with the relevant files for their respective test cases. If possible, incorporate integration or e2e tests. For guidance, refer to the ",(0,o.kt)("inlineCode",{parentName:"p"},"flipper")," template in ",(0,o.kt)("a",{parentName:"p",href:"https://github.com/CoinFabrik/scout-soroban/tree/main/templates/test-case"},(0,o.kt)("inlineCode",{parentName:"a"},"templates/test-case")),"."))))}d.isMDXComponent=!0}}]);