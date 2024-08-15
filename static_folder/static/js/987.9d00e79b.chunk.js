"use strict";(self.webpackChunkmantis_free_react_admin_template=self.webpackChunkmantis_free_react_admin_template||[]).push([[987],{7070:(i,e,t)=>{t.d(e,{A:()=>M});var r=t(91671),a=t(3123),n=t(54858),s=t(33747),d=t(10712),l=t(83864),o=t(38030);function h(i){return String(i).match(/[\d.\-+]*\s*(.*)/)[1]||""}function c(i){return parseFloat(i)}var u=t(15843),x=t(7888),m=t(32285),g=t(17392),A=t(87968);function j(i){return(0,A.Ay)("MuiSkeleton",i)}(0,g.A)("MuiSkeleton",["root","text","rectangular","rounded","circular","pulse","wave","withChildren","fitContent","heightAuto"]);var p,v,b,y,H=t(84463);const S=["animation","className","component","height","style","variant","width"];let f,w,L,B;const R=(0,l.i7)(f||(f=p||(p=(0,a.A)(["\n  0% {\n    opacity: 1;\n  }\n\n  50% {\n    opacity: 0.4;\n  }\n\n  100% {\n    opacity: 1;\n  }\n"])))),C=(0,l.i7)(w||(w=v||(v=(0,a.A)(["\n  0% {\n    transform: translateX(-100%);\n  }\n\n  50% {\n    /* +0.5s of delay between each loop */\n    transform: translateX(100%);\n  }\n\n  100% {\n    transform: translateX(100%);\n  }\n"])))),k=(0,x.Ay)("span",{name:"MuiSkeleton",slot:"Root",overridesResolver:(i,e)=>{const{ownerState:t}=i;return[e.root,e[t.variant],!1!==t.animation&&e[t.animation],t.hasChildren&&e.withChildren,t.hasChildren&&!t.width&&e.fitContent,t.hasChildren&&!t.height&&e.heightAuto]}})((i=>{let{theme:e,ownerState:t}=i;const r=h(e.shape.borderRadius)||"px",a=c(e.shape.borderRadius);return(0,s.A)({display:"block",backgroundColor:e.vars?e.vars.palette.Skeleton.bg:(0,u.X4)(e.palette.text.primary,"light"===e.palette.mode?.11:.13),height:"1.2em"},"text"===t.variant&&{marginTop:0,marginBottom:0,height:"auto",transformOrigin:"0 55%",transform:"scale(1, 0.60)",borderRadius:"".concat(a).concat(r,"/").concat(Math.round(a/.6*10)/10).concat(r),"&:empty:before":{content:'"\\00a0"'}},"circular"===t.variant&&{borderRadius:"50%"},"rounded"===t.variant&&{borderRadius:(e.vars||e).shape.borderRadius},t.hasChildren&&{"& > *":{visibility:"hidden"}},t.hasChildren&&!t.width&&{maxWidth:"fit-content"},t.hasChildren&&!t.height&&{height:"auto"})}),(i=>{let{ownerState:e}=i;return"pulse"===e.animation&&(0,l.AH)(L||(L=b||(b=(0,a.A)(["\n      animation: "," 2s ease-in-out 0.5s infinite;\n    "]))),R)}),(i=>{let{ownerState:e,theme:t}=i;return"wave"===e.animation&&(0,l.AH)(B||(B=y||(y=(0,a.A)(["\n      position: relative;\n      overflow: hidden;\n\n      /* Fix bug in Safari https://bugs.webkit.org/show_bug.cgi?id=68196 */\n      -webkit-mask-image: -webkit-radial-gradient(white, black);\n\n      &::after {\n        animation: "," 2s linear 0.5s infinite;\n        background: linear-gradient(\n          90deg,\n          transparent,\n          ",",\n          transparent\n        );\n        content: '';\n        position: absolute;\n        transform: translateX(-100%); /* Avoid flash during server-side hydration */\n        bottom: 0;\n        left: 0;\n        right: 0;\n        top: 0;\n      }\n    "]))),C,(t.vars||t).palette.action.hover)})),q=r.forwardRef((function(i,e){const t=(0,m.A)({props:i,name:"MuiSkeleton"}),{animation:r="pulse",className:a,component:l="span",height:h,style:c,variant:u="text",width:x}=t,g=(0,n.A)(t,S),A=(0,s.A)({},t,{animation:r,component:l,variant:u,hasChildren:Boolean(g.children)}),p=(i=>{const{classes:e,variant:t,animation:r,hasChildren:a,width:n,height:s}=i,d={root:["root",t,r,a&&"withChildren",a&&!n&&"fitContent",a&&!s&&"heightAuto"]};return(0,o.A)(d,j,e)})(A);return(0,H.jsx)(k,(0,s.A)({as:l,ref:e,className:(0,d.A)(p.root,a),ownerState:A},g,{style:(0,s.A)({width:x,height:h},c)}))}));var W=t(86746),z=t(82847),F=t(97094);const M=i=>{let{children:e}=i;const[t,a]=(0,r.useState)(!0);(0,r.useEffect)((()=>{a(!1)}),[]);const n=(0,H.jsx)(F.A,{title:(0,H.jsx)(q,{sx:{width:{xs:120,md:180}}}),secondary:(0,H.jsx)(q,{animation:"wave",variant:"circular",width:24,height:24}),children:(0,H.jsxs)(W.A,{spacing:1,children:[(0,H.jsx)(q,{}),(0,H.jsx)(q,{sx:{height:64},animation:"wave",variant:"rectangular"}),(0,H.jsx)(q,{}),(0,H.jsx)(q,{})]})});return(0,H.jsxs)(H.Fragment,{children:[t&&(0,H.jsxs)(z.Ay,{container:!0,spacing:3,children:[(0,H.jsx)(z.Ay,{item:!0,xs:12,md:6,children:n}),(0,H.jsx)(z.Ay,{item:!0,xs:12,md:6,children:n}),(0,H.jsx)(z.Ay,{item:!0,xs:12,md:6,children:n}),(0,H.jsx)(z.Ay,{item:!0,xs:12,md:6,children:n})]}),!t&&e]})}},92987:(i,e,t)=>{t.r(e),t.d(e,{default:()=>u});var r=t(82847),a=t(86746),n=t(43276),s=t(72833),d=t(87007),l=t(58374),o=t(7070),h=t(97094),c=t(84463);const u=()=>(0,c.jsx)(o.A,{children:(0,c.jsxs)(r.Ay,{container:!0,spacing:3,children:[(0,c.jsx)(r.Ay,{item:!0,xs:12,lg:6,children:(0,c.jsxs)(a.A,{spacing:3,children:[(0,c.jsx)(h.A,{title:"Basic",codeHighlight:!0,children:(0,c.jsxs)(a.A,{spacing:.75,sx:{mt:-1.5},children:[(0,c.jsx)(n.A,{variant:"h1",children:"Inter"}),(0,c.jsx)(n.A,{variant:"h5",children:"Font Family"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Medium"}),(0,c.jsx)(n.A,{variant:"h6",children:"Bold"})]})]})}),(0,c.jsx)(h.A,{title:"Heading",codeHighlight:!0,children:(0,c.jsxs)(a.A,{spacing:2,children:[(0,c.jsx)(n.A,{variant:"h1",children:"H1 Heading"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 38px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Bold"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 46px"})]}),(0,c.jsx)(d.A,{}),(0,c.jsx)(n.A,{variant:"h2",children:"H2 Heading"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 30px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Bold"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 38px"})]}),(0,c.jsx)(d.A,{}),(0,c.jsx)(n.A,{variant:"h3",children:"H3 Heading"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 24px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular & Bold"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 32px"})]}),(0,c.jsx)(d.A,{}),(0,c.jsx)(n.A,{variant:"h4",children:"H4 Heading"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 20px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Bold"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 28px"})]}),(0,c.jsx)(d.A,{}),(0,c.jsx)(n.A,{variant:"h5",children:"H5 Heading"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 16px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular & Medium & Bold"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 24px"})]}),(0,c.jsx)(d.A,{}),(0,c.jsx)(n.A,{variant:"h6",children:"H6 Heading / Subheading"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 14px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 22px"})]})]})}),(0,c.jsx)(h.A,{title:"Body 1",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"body1",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 14px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 22px"})]})]})}),(0,c.jsx)(h.A,{title:"Body 2",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"body2",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 12px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 20px"})]})]})}),(0,c.jsx)(h.A,{title:"Subtitle 1",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"subtitle1",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 14px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Medium"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 22px"})]})]})}),(0,c.jsx)(h.A,{title:"Subtitle 2",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"subtitle2",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 12px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Medium"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 20px"})]})]})}),(0,c.jsx)(h.A,{title:"Caption",codeHighlight:!0,children:(0,c.jsxs)(a.A,{spacing:1,children:[(0,c.jsx)(n.A,{variant:"caption",children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 12px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 20px"})]})]})})]})}),(0,c.jsx)(r.Ay,{item:!0,xs:12,lg:6,children:(0,c.jsxs)(a.A,{spacing:3,children:[(0,c.jsx)(h.A,{title:"Alignment",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"body2",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsx)(n.A,{variant:"body2",textAlign:"center",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsx)(n.A,{variant:"body2",textAlign:"right",children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."})]})}),(0,c.jsx)(h.A,{title:"Gutter Bottom",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"body1",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsx)(n.A,{variant:"body2",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 12px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 20px"})]})]})}),(0,c.jsx)(h.A,{title:"Overline",codeHighlight:!0,children:(0,c.jsxs)(a.A,{spacing:1.5,children:[(0,c.jsx)(n.A,{variant:"overline",children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 12px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 20px"})]})]})}),(0,c.jsx)(h.A,{title:"Link",codeHighlight:!0,children:(0,c.jsxs)(a.A,{spacing:1.5,children:[(0,c.jsx)(l.A,{href:"#",children:"www.mantis.com"}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 12px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 20px"})]})]})}),(0,c.jsx)(h.A,{title:"Colors",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"h6",color:"textPrimary",gutterBottom:!0,children:"This is textPrimary text color."}),(0,c.jsx)(n.A,{variant:"h6",color:"textSecondary",gutterBottom:!0,children:"This is textSecondary text color."}),(0,c.jsx)(n.A,{variant:"h6",color:"primary",gutterBottom:!0,children:"This is primary text color."}),(0,c.jsx)(n.A,{variant:"h6",color:"secondary",gutterBottom:!0,children:"This is secondary text color."}),(0,c.jsx)(n.A,{variant:"h6",color:"success",gutterBottom:!0,children:"This is success text color."}),(0,c.jsx)(n.A,{variant:"h6",sx:{color:"warning.main"},gutterBottom:!0,children:"This is warning text color."}),(0,c.jsx)(n.A,{variant:"h6",color:"error",gutterBottom:!0,children:"This is error text color."})]})}),(0,c.jsx)(h.A,{title:"Paragraph",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"body1",gutterBottom:!0,children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 14px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Regular"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 22px"})]})]})}),(0,c.jsx)(h.A,{title:"Font Style",codeHighlight:!0,children:(0,c.jsxs)(c.Fragment,{children:[(0,c.jsx)(n.A,{variant:"body1",gutterBottom:!0,sx:{fontStyle:"italic"},children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsx)(n.A,{variant:"subtitle1",gutterBottom:!0,sx:{fontStyle:"italic"},children:"Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}),(0,c.jsxs)(s.A,{"aria-label":"breadcrumb",children:[(0,c.jsx)(n.A,{variant:"h6",children:"Size: 14px"}),(0,c.jsx)(n.A,{variant:"h6",children:"Weight: Italic Regular & Italic Bold"}),(0,c.jsx)(n.A,{variant:"h6",children:"Line Height: 22px"})]})]})})]})})]})})}}]);