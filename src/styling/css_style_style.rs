use yew::prelude::*;

#[function_component(StyleString)]
pub fn style_string() -> Html {
    let style_string: &str = ".main-container {
    max-width: 910px;
    margin-left: auto;
    margin-right: auto;
    font-family: \"Carlito\";
    font-family: \"sans-serif\";
  }
  div#header {
    height: 0px;
    width: 100%;
    margin: 0;
  }
  
  h1 {
    font-size: 2.5em;
    font-family: \"Carlito\", \"sans-serif\";
    margin-left: 40px;
    margin-top: 50px;
  }
  
  div#header-footer {
    background-color: #e30613;
    padding-top: 45px;
    color: white;
    height: 350px;
    width: 100%;
    margin: 0;
    font-family: \"Carlito\";
  }
  
  .text {
    font-weight: bold;
    font-family: \"Carlito\", \"sans-serif\";
  }
  
  .main-image{
    margin-bottom: 20px;
    margin-top: 10px;
    width: 95%;
    display:block;
    margin-left:auto;
    margin-right:auto;
  }
  
  .image-artikel {
    margin-left: 5%;
    margin-right: 5%;
    width: auto;
    height : auto;
    float: none;
    display: block;
    text-align: center;
    max-width:300px;
  }
  
  .image-artikel-rechts {
    margin-left: auto;
    margin-right: auto;
    width: 80%;
    height : auto;
    float: center;
    display: block;
    text-align: center;
    max-width:300px;
  }
  
  .image-artikel-links {
    margin-left: auto;
    margin-right: auto;
    width: 80%;
    height : auto;
    float: center;
    display: block;
    text-align: center;
    max-width:300px;
  }
  
  [class*=\"image-header-footer\"] {
    margin-left: auto;
    margin-right: auto;
    margin-top: 30px;
    width: 75%;
    height: auto;
    display: block;
    float: center;
  }
  
  .einleitung-header {
    font-size: 1.875em;
    text-align: center;
  }
  
  .sonstige-header {
    font-size: 1.875em;
  }
  
  .trenner-mitte {
    background-color: #e30613;
    color: white;
    height: 20px;
    width: 100%;
    margin: 0;
    overflow: hidden;
    margin-bottom: 30px;
  }
  
  .trenner-artikel {
    background-color: white;
    color: white;
    height: 10px;
    width: 100%;
    margin: 0;
    overflow: hidden;
    margin-bottom: 10px;
  }
  
  .btnStack {
    font-family: Carlito;
    background-color: #e30613;
    color: white;
    text-decoration: none;
    display: inline-block;
    padding: 6px 12px;
    margin-bottom: 0;
    font-size: 1em;
    font-weight: normal;
    line-height: 1.428571429;
    text-align: center;
    white-space: nowrap;
    vertical-align: middle;
    cursor: pointer;
    border: 1px solid transparent;
    border-radius: 4px;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    -o-user-select: none;
    user-select: none;
  }

  a.btnStack:hover {
    background-color: #000;
  }
   
  .link-button {
    color: white;
    text-align: right;
  }
  
  a:link {
    color: white;
    background-color: #e30613;
    text-decoration: none;
  }l
   
  a:visited {
    color: white;
    background-color: #e30613;
    text-decoration: none;
  }
   
  a:hover {
    color: white;
    background-color: #e30613;
    text-decoration: underline;
  }
   
  a:active {
    color: white;
    background-color: #e30613;
    text-decoration: underline;
  }
  
  [class*=\"kapitel-text\"] {
    width: 90%;
    margin-left: 5%;
    margin-right: 5%;
    height : auto;
    vertical-align: bot;
    float : center;
  }
    
  .news-text {
    width: 100%;
    font-size: 2.5em;
    margin-left: auto;
    margin-right: auto;
    margin-top: 5px;
    height : auto;
    display: inline-block;
    vertical-align: middle;
    text-align: center;
    float: center;
  }
    
    
  [class*=\"impressum\"] {
    margin-left: auto;
    margin-right: auto;
    text-align: center;
  }
  
  .impressum {
    color: grey;
    margin-left: 20px;
    font-family: \"Carlito\";
  }
  
  div.impressum a:link {
    color: grey;
    background-color: white;
    text-decoration: underline grey;
  }
  
  div.impressum a:hover {
    color: grey;
    background-color: white;
    text-decoration: underline grey;
  }
  
  div.impressum a:visited {
    color: #654321;
    background-color: white;
    text-decoration: underline grey;
  }
  
  .links {
     margin-top: 0px;
     margin-bottom: 20px;
     width: 100%;
     text-align: center;
     float: center;
     display:inline-block;
  }
  
  @media only screen and (min-width: 900px) {
    /* For mobile phones: */
    
    .image-artikel {
      margin-left: 5%;
      margin-right: 5%;
      width: auto;
      height : auto;
      float: none;
      display: block;
    }
    
    .image-artikel-rechts {
      margin-right: 20px;
      margin-left: 20px;
      width : 30%;
      height : auto;
      float : right;
      display : inline;
    }
    
    .image-artikel-links {
      margin-right: 20px;
      margin-left: 20px;
      width : 30%;
      height : auto;
      float : left;
      display : inline;
    }
    
    .kapitel-text-links {
      width: 55%;
      display: inline-block;
      margin-left: 20px;
      margin-right: 20px;
      overflow: hidden;
      float : left;
    }
    
    .kapitel-text-rechts {
      width: 55%;
      display: inline-block;
      margin-left: 20px;
      margin-right: 20px;
      overflow: hidden;
      float : right;
    }
  
    [class=\"links\"] {
       margin-left: 40px;
       margin-top: 80px;
       margin-bottom: 0px;
       float: left;
       width: 50%;
       vertical-align: left;
       text-align: left;
       display: inline;
    }
    
    [class*=\"news-text\"]  {
      font-size: 3.75em;
      font-family: \"Carlito\", \"sans-serif\";
      margin-left: 40px;
      margin-top: 20px;
      float: left;
      width: 50%;
      vertical-align: left;
      text-align: left;
      display: inline;
    }
    
    .image-header-footer {
      margin-right: 40px;
      margin-top: 30px;
      margin-left: 0;
      width : 30%;
      height : auto;
      float : right;
      position: inline;
      display: inline;
    }
    
    div#header-footer {
      background-color: #e30613;
      color: white;
      height: 140px;
      width: 100%;
      margin: 0;
      overflow: hidden;
      font-family: \"Carlito\";
      border-top: 0px;
      padding-top: 0px;
    }
  }";

  html! {
    <style>
        {style_string}
    </style>
  }
}