use time::PreciseTime;
use actix_web::{web, App, HttpResponse, HttpServer};
use sudoku_solver::WebForm;

fn main(){
    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
        .route("/result", web::get().to(result))
    })
    .bind("0.0.0.0:8000")
    .expect("Cannot bind to port 8000")
    .run()
    .unwrap();
}

fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    
    <title>Sudoku Solver</title>
    
    <link href=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.6/css/bootstrap.min.css\" rel=\"stylesheet\">
    
    <style type=\"text/css\">
    
      html, body {
        background-color: #FAFAFA
      }

      table {
        border: 2px solid #000000;
      }

      td {
        border: 1px solid #000000;
        text-align: center;
        vertical-align: middle;  
      }

      input {
        color: #000000;
        padding: 0;
        border: 0;
        text-align: center;
        width: 48px;
        height: 48px;
        font-size: 24px;
        background-color: #FFFFFF;
        outline: none;
      }

      input:disabled {
        background-color: #EEEEEE;
      }

      #cell_0,  #cell_1,  #cell_2  { border-top:    2px solid #000000; }
      #cell_2,  #cell_11, #cell_20 { border-right:  2px solid #000000; }
      #cell_18, #cell_19, #cell_20 { border-bottom: 2px solid #000000; }
      #cell_0,  #cell_9,  #cell_18 { border-left:   2px solid #000000; }

      #cell_3,  #cell_4,  #cell_5  { border-top:    2px solid #000000; }
      #cell_5,  #cell_14, #cell_23 { border-right:  2px solid #000000; }
      #cell_21, #cell_22, #cell_23 { border-bottom: 2px solid #000000; }
      #cell_3,  #cell_12, #cell_21 { border-left:   2px solid #000000; }

      #cell_6,  #cell_7,  #cell_8  { border-top:    2px solid #000000; }
      #cell_8,  #cell_17, #cell_26 { border-right:  2px solid #000000; }
      #cell_24, #cell_25, #cell_26 { border-bottom: 2px solid #000000; }
      #cell_6,  #cell_15, #cell_24 { border-left:   2px solid #000000; }

      #cell_27, #cell_28, #cell_29 { border-top:    2px solid #000000; }
      #cell_29, #cell_38, #cell_47 { border-right:  2px solid #000000; }
      #cell_45, #cell_46, #cell_47 { border-bottom: 2px solid #000000; }
      #cell_27, #cell_36, #cell_45 { border-left:   2px solid #000000; }

      #cell_30, #cell_31, #cell_32 { border-top:    2px solid #000000; }
      #cell_32, #cell_41, #cell_50 { border-right:  2px solid #000000; }
      #cell_48, #cell_49, #cell_50 { border-bottom: 2px solid #000000; }
      #cell_30, #cell_39, #cell_48 { border-left:   2px solid #000000; }

      #cell_33, #cell_34, #cell_35 { border-top:    2px solid #000000; }
      #cell_35, #cell_44, #cell_53 { border-right:  2px solid #000000; }
      #cell_51, #cell_52, #cell_53 { border-bottom: 2px solid #000000; }
      #cell_33, #cell_42, #cell_51 { border-left:   2px solid #000000; }

      #cell_54, #cell_55, #cell_56 { border-top:    2px solid #000000; }
      #cell_56, #cell_65, #cell_74 { border-right:  2px solid #000000; }
      #cell_72, #cell_73, #cell_74 { border-bottom: 2px solid #000000; }
      #cell_54, #cell_63, #cell_72 { border-left:   2px solid #000000; }

      #cell_57, #cell_58, #cell_59 { border-top:    2px solid #000000; }
      #cell_59, #cell_68, #cell_77 { border-right:  2px solid #000000; }
      #cell_75, #cell_76, #cell_77 { border-bottom: 2px solid #000000; }
      #cell_57, #cell_66, #cell_75 { border-left:   2px solid #000000; }

      #cell_60, #cell_61, #cell_62 { border-top:    2px solid #000000; }
      #cell_62, #cell_71, #cell_80 { border-right:  2px solid #000000; }
      #cell_78, #cell_79, #cell_80 { border-bottom: 2px solid #000000; }
      #cell_60, #cell_69, #cell_78 { border-left:   2px solid #000000; }

    </style>
  </head>
  <body>

    <div class=\"container\">
      
      <h1>Sudoku Solver</h1>

      <form action=\"/result\" method=\"get\">

      <table id=\"grid\">

        <tr>
          <td><input maxlength=\"1\" name=\"cell_0\" id=\"cell_0\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_1\" id=\"cell_1\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_2\" id=\"cell_2\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_3\" id=\"cell_3\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_4\" id=\"cell_4\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_5\" id=\"cell_5\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_6\" id=\"cell_6\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_7\" id=\"cell_7\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_8\" id=\"cell_8\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>
          <td><input maxlength=\"1\" name=\"cell_9\" id=\"cell_9\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_10\" id=\"cell_10\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_11\" id=\"cell_11\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_12\" id=\"cell_12\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_13\" id=\"cell_13\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_14\" id=\"cell_14\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_15\" id=\"cell_15\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_16\" id=\"cell_16\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_17\" id=\"cell_17\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_18\" id=\"cell_18\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_19\" id=\"cell_19\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_20\" id=\"cell_20\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_21\" id=\"cell_21\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_22\" id=\"cell_22\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_23\" id=\"cell_23\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_24\" id=\"cell_24\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_25\" id=\"cell_25\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_26\" id=\"cell_26\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_27\" id=\"cell_27\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_28\" id=\"cell_28\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_29\" id=\"cell_29\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_30\" id=\"cell_30\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_31\" id=\"cell_31\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_32\" id=\"cell_32\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_33\" id=\"cell_33\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_34\" id=\"cell_34\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_35\" id=\"cell_35\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_36\" id=\"cell_36\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_37\" id=\"cell_37\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_38\" id=\"cell_38\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_39\" id=\"cell_39\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_40\" id=\"cell_40\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_41\" id=\"cell_41\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_42\" id=\"cell_42\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_43\" id=\"cell_43\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_44\" id=\"cell_44\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_45\" id=\"cell_45\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_46\" id=\"cell_46\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_47\" id=\"cell_47\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_48\" id=\"cell_48\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_49\" id=\"cell_49\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_50\" id=\"cell_50\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_51\" id=\"cell_51\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_52\" id=\"cell_52\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_53\" id=\"cell_53\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_54\" id=\"cell_54\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_55\" id=\"cell_55\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_56\" id=\"cell_56\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_57\" id=\"cell_57\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_58\" id=\"cell_58\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_59\" id=\"cell_59\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_60\" id=\"cell_60\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_61\" id=\"cell_61\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_62\" id=\"cell_62\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_63\" id=\"cell_63\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_64\" id=\"cell_64\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_65\" id=\"cell_65\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_66\" id=\"cell_66\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_67\" id=\"cell_67\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_68\" id=\"cell_68\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_69\" id=\"cell_69\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_70\" id=\"cell_70\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_71\" id=\"cell_71\" type=\"number\" value=\"0\"></td>
        </tr>

        <tr>          
          <td><input maxlength=\"1\" name=\"cell_72\" id=\"cell_72\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_73\" id=\"cell_73\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_74\" id=\"cell_74\" type=\"number\" value=\"0\"></td>
        
          <td><input maxlength=\"1\" name=\"cell_75\" id=\"cell_75\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_76\" id=\"cell_76\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_77\" id=\"cell_77\" type=\"number\" value=\"0\"></td>
          
          <td><input maxlength=\"1\" name=\"cell_78\" id=\"cell_78\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_79\" id=\"cell_79\" type=\"number\" value=\"0\"></td>
          <td><input maxlength=\"1\" name=\"cell_80\" id=\"cell_80\" type=\"number\" value=\"0\"></td>
        </tr>

      </table>
      <input type=\"submit\" value=\"Submit\">
    </form>

    </div>
    <br />
    <div>
    	Borrowed this html Sudoku board from <a href=\"https://gist.github.com/thebinarypenguin/4d45ffe87096e508800b5d11544bf2fa\">
      thebinarypenguin's github</a> with some modifications. The html and css are all originally his/her work.
      I'm using this WITHOUT permission. I couldn't find this file's copyright information.<br />
      Server side software is written entirely in Rust by me (except, of course, std library, web crate, time crate, etc.)
    </div>

  </body>
</html>")
}

// fn result(form: web::Query<WebForm>) -> HttpResponse {

fn result(form: web::Query<WebForm>) -> HttpResponse {
  let start = PreciseTime::now();

  let puzzle_result = form.convert_to_big_box();
  match puzzle_result {
    Some(mut puzzle) => {
      puzzle.solve_puzzle();
      let stop = PreciseTime::now();
      HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\">
    <meta http-equiv=\"X-UA-Compatible\" content=\"IE=edge\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    
    <title>Sudoku Solver - Result</title>
    
    <link href=\"https://maxcdn.bootstrapcdn.com/bootstrap/3.3.6/css/bootstrap.min.css\" rel=\"stylesheet\">
    
    <style type=\"text/css\">
    
      html, body {{
        background-color: #FAFAFA
      }}

      table {{
        border: 2px solid #000000;
      }}

      td {{
        border: 1px solid #000000;
        text-align: center;
        vertical-align: middle;  
      }}

      input {{
        color: #000000;
        padding: 0;
        border: 0;
        text-align: center;
        width: 48px;
        height: 48px;
        font-size: 24px;
        background-color: #FFFFFF;
        outline: none;
      }}

      input:disabled {{
        background-color: #EEEEEE;
      }}

      .notes {{
        color: #000000;
        padding: 0;
        border: 0;
        width: 48px;
        height: 48px;
        font-size: 12px;
        background-color: #FFFFFF;
        outline: none;
      }}

      #cell_0,  #cell_1,  #cell_2  {{ border-top:    2px solid #000000; }}
      #cell_2,  #cell_11, #cell_20 {{ border-right:  2px solid #000000; }}
      #cell_18, #cell_19, #cell_20 {{ border-bottom: 2px solid #000000; }}
      #cell_0,  #cell_9,  #cell_18 {{ border-left:   2px solid #000000; }}

      #cell_3,  #cell_4,  #cell_5  {{ border-top:    2px solid #000000; }}
      #cell_5,  #cell_14, #cell_23 {{ border-right:  2px solid #000000; }}
      #cell_21, #cell_22, #cell_23 {{ border-bottom: 2px solid #000000; }}
      #cell_3,  #cell_12, #cell_21 {{ border-left:   2px solid #000000; }}

      #cell_6,  #cell_7,  #cell_8  {{ border-top:    2px solid #000000; }}
      #cell_8,  #cell_17, #cell_26 {{ border-right:  2px solid #000000; }}
      #cell_24, #cell_25, #cell_26 {{ border-bottom: 2px solid #000000; }}
      #cell_6,  #cell_15, #cell_24 {{ border-left:   2px solid #000000; }}

      #cell_27, #cell_28, #cell_29 {{ border-top:    2px solid #000000; }}
      #cell_29, #cell_38, #cell_47 {{ border-right:  2px solid #000000; }}
      #cell_45, #cell_46, #cell_47 {{ border-bottom: 2px solid #000000; }}
      #cell_27, #cell_36, #cell_45 {{ border-left:   2px solid #000000; }}

      #cell_30, #cell_31, #cell_32 {{ border-top:    2px solid #000000; }}
      #cell_32, #cell_41, #cell_50 {{ border-right:  2px solid #000000; }}
      #cell_48, #cell_49, #cell_50 {{ border-bottom: 2px solid #000000; }}
      #cell_30, #cell_39, #cell_48 {{ border-left:   2px solid #000000; }}

      #cell_33, #cell_34, #cell_35 {{ border-top:    2px solid #000000; }}
      #cell_35, #cell_44, #cell_53 {{ border-right:  2px solid #000000; }}
      #cell_51, #cell_52, #cell_53 {{ border-bottom: 2px solid #000000; }}
      #cell_33, #cell_42, #cell_51 {{ border-left:   2px solid #000000; }}

      #cell_54, #cell_55, #cell_56 {{ border-top:    2px solid #000000; }}
      #cell_56, #cell_65, #cell_74 {{ border-right:  2px solid #000000; }}
      #cell_72, #cell_73, #cell_74 {{ border-bottom: 2px solid #000000; }}
      #cell_54, #cell_63, #cell_72 {{ border-left:   2px solid #000000; }}

      #cell_57, #cell_58, #cell_59 {{ border-top:    2px solid #000000; }}
      #cell_59, #cell_68, #cell_77 {{ border-right:  2px solid #000000; }}
      #cell_75, #cell_76, #cell_77 {{ border-bottom: 2px solid #000000; }}
      #cell_57, #cell_66, #cell_75 {{ border-left:   2px solid #000000; }}

      #cell_60, #cell_61, #cell_62 {{ border-top:    2px solid #000000; }}
      #cell_62, #cell_71, #cell_80 {{ border-right:  2px solid #000000; }}
      #cell_78, #cell_79, #cell_80 {{ border-bottom: 2px solid #000000; }}
      #cell_60, #cell_69, #cell_78 {{ border-left:   2px solid #000000; }}

    </style>
  </head>
  <body>

    <div class=\"container\">
      
      <h1>Sudoku Solver - Result</h1>
      <div>
      It took {:.3} second(s) to solve this puzzle.
      </div>
      <div>
      You can solve another by <a href=\"/\">going back to the main page</a>.
      </div>
      <br />
      <table id=\"grid\">

        <tr>
          <td id=\"cell_0\">{}</td>
          <td id=\"cell_1\">{}</td>
          <td id=\"cell_2\">{}</td>
          
          <td id=\"cell_3\">{}</td>
          <td id=\"cell_4\">{}</td>
          <td id=\"cell_5\">{}</td>
          
          <td id=\"cell_6\">{}</td>
          <td id=\"cell_7\">{}</td>
          <td id=\"cell_8\">{}</td>
        </tr>

        <tr>
          <td id=\"cell_9\">{}</td>
          <td id=\"cell_10\">{}</td>
          <td id=\"cell_11\">{}</td>
          
          <td id=\"cell_12\">{}</td>
          <td id=\"cell_13\">{}</td>
          <td id=\"cell_14\">{}</td>
          
          <td id=\"cell_15\">{}</td>
          <td id=\"cell_16\">{}</td>
          <td id=\"cell_17\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_18\">{}</td>
          <td id=\"cell_19\">{}</td>
          <td id=\"cell_20\">{}</td>
          
          <td id=\"cell_21\">{}</td>
          <td id=\"cell_22\">{}</td>
          <td id=\"cell_23\">{}</td>
          
          <td id=\"cell_24\">{}</td>
          <td id=\"cell_25\">{}</td>
          <td id=\"cell_26\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_27\">{}</td>
          <td id=\"cell_28\">{}</td>
          <td id=\"cell_29\">{}</td>
          
          <td id=\"cell_30\">{}</td>
          <td id=\"cell_31\">{}</td>
          <td id=\"cell_32\">{}</td>
          
          <td id=\"cell_33\">{}</td>
          <td id=\"cell_34\">{}</td>
          <td id=\"cell_35\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_36\">{}</td>
          <td id=\"cell_37\">{}</td>
          <td id=\"cell_38\">{}</td>
          
          <td id=\"cell_39\">{}</td>
          <td id=\"cell_40\">{}</td>
          <td id=\"cell_41\">{}</td>
          
          <td id=\"cell_42\">{}</td>
          <td id=\"cell_43\">{}</td>
          <td id=\"cell_44\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_45\">{}</td>
          <td id=\"cell_46\">{}</td>
          <td id=\"cell_47\">{}</td>
          
          <td id=\"cell_48\">{}</td>
          <td id=\"cell_49\">{}</td>
          <td id=\"cell_50\">{}</td>
          
          <td id=\"cell_51\">{}</td>
          <td id=\"cell_52\">{}</td>
          <td id=\"cell_53\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_54\">{}</td>
          <td id=\"cell_55\">{}</td>
          <td id=\"cell_56\">{}</td>
          
          <td id=\"cell_57\">{}</td>
          <td id=\"cell_58\">{}</td>
          <td id=\"cell_59\">{}</td>
          
          <td id=\"cell_60\">{}</td>
          <td id=\"cell_61\">{}</td>
          <td id=\"cell_62\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_63\">{}</td>
          <td id=\"cell_64\">{}</td>
          <td id=\"cell_65\">{}</td>
          
          <td id=\"cell_66\">{}</td>
          <td id=\"cell_67\">{}</td>
          <td id=\"cell_68\">{}</td>
          
          <td id=\"cell_69\">{}</td>
          <td id=\"cell_70\">{}</td>
          <td id=\"cell_71\">{}</td>
        </tr>

        <tr>          
          <td id=\"cell_72\">{}</td>
          <td id=\"cell_73\">{}</td>
          <td id=\"cell_74\">{}</td>
        
          <td id=\"cell_75\">{}</td>
          <td id=\"cell_76\">{}</td>
          <td id=\"cell_77\">{}</td>
          
          <td id=\"cell_78\">{}</td>
          <td id=\"cell_79\">{}</td>
          <td id=\"cell_80\">{}</td>
        </tr>

      </table>
      <br /><br />
    </div>
    <div>
      Borrowed this html Sudoku board from <a href=\"https://gist.github.com/thebinarypenguin/4d45ffe87096e508800b5d11544bf2fa\">
      thebinarypenguin's github</a> with some modifications. The html and css are all originally his/her work.
      I'm using this WITHOUT permission. I couldn't find this file's copyright information.<br />
      Server side software is written entirely in Rust by me (except, of course, std library, web crate, time crate, etc.)
    </div>

  </body>
</html>",
        start.to(stop).num_milliseconds() as f64 / 1000 as f64,
        puzzle.little_boxes[0].pretty_print(), puzzle.little_boxes[1].pretty_print(), puzzle.little_boxes[2].pretty_print(), 
        puzzle.little_boxes[3].pretty_print(), puzzle.little_boxes[4].pretty_print(), puzzle.little_boxes[5].pretty_print(), 
        puzzle.little_boxes[6].pretty_print(), puzzle.little_boxes[7].pretty_print(), puzzle.little_boxes[8].pretty_print(), 
        puzzle.little_boxes[9].pretty_print(), puzzle.little_boxes[10].pretty_print(), puzzle.little_boxes[11].pretty_print(), 
        puzzle.little_boxes[12].pretty_print(), puzzle.little_boxes[13].pretty_print(), puzzle.little_boxes[14].pretty_print(), 
        puzzle.little_boxes[15].pretty_print(), puzzle.little_boxes[16].pretty_print(), puzzle.little_boxes[17].pretty_print(), 
        puzzle.little_boxes[18].pretty_print(), puzzle.little_boxes[19].pretty_print(), puzzle.little_boxes[20].pretty_print(), 
        puzzle.little_boxes[21].pretty_print(), puzzle.little_boxes[22].pretty_print(), puzzle.little_boxes[23].pretty_print(), 
        puzzle.little_boxes[24].pretty_print(), puzzle.little_boxes[25].pretty_print(), puzzle.little_boxes[26].pretty_print(), 
        puzzle.little_boxes[27].pretty_print(), puzzle.little_boxes[28].pretty_print(), puzzle.little_boxes[29].pretty_print(), 
        puzzle.little_boxes[30].pretty_print(), puzzle.little_boxes[31].pretty_print(), puzzle.little_boxes[32].pretty_print(), 
        puzzle.little_boxes[33].pretty_print(), puzzle.little_boxes[34].pretty_print(), puzzle.little_boxes[35].pretty_print(), 
        puzzle.little_boxes[36].pretty_print(), puzzle.little_boxes[37].pretty_print(), puzzle.little_boxes[38].pretty_print(), 
        puzzle.little_boxes[39].pretty_print(), puzzle.little_boxes[40].pretty_print(), puzzle.little_boxes[41].pretty_print(), 
        puzzle.little_boxes[42].pretty_print(), puzzle.little_boxes[43].pretty_print(), puzzle.little_boxes[44].pretty_print(), 
        puzzle.little_boxes[45].pretty_print(), puzzle.little_boxes[46].pretty_print(), puzzle.little_boxes[47].pretty_print(), 
        puzzle.little_boxes[48].pretty_print(), puzzle.little_boxes[49].pretty_print(), puzzle.little_boxes[50].pretty_print(), 
        puzzle.little_boxes[51].pretty_print(), puzzle.little_boxes[52].pretty_print(), puzzle.little_boxes[53].pretty_print(), 
        puzzle.little_boxes[54].pretty_print(), puzzle.little_boxes[55].pretty_print(), puzzle.little_boxes[56].pretty_print(), 
        puzzle.little_boxes[57].pretty_print(), puzzle.little_boxes[58].pretty_print(), puzzle.little_boxes[59].pretty_print(), 
        puzzle.little_boxes[60].pretty_print(), puzzle.little_boxes[61].pretty_print(), puzzle.little_boxes[62].pretty_print(), 
        puzzle.little_boxes[63].pretty_print(), puzzle.little_boxes[64].pretty_print(), puzzle.little_boxes[65].pretty_print(), 
        puzzle.little_boxes[66].pretty_print(), puzzle.little_boxes[67].pretty_print(), puzzle.little_boxes[68].pretty_print(), 
        puzzle.little_boxes[69].pretty_print(), puzzle.little_boxes[70].pretty_print(), puzzle.little_boxes[71].pretty_print(), 
        puzzle.little_boxes[72].pretty_print(), puzzle.little_boxes[73].pretty_print(), puzzle.little_boxes[74].pretty_print(), 
        puzzle.little_boxes[75].pretty_print(), puzzle.little_boxes[76].pretty_print(), puzzle.little_boxes[77].pretty_print(), 
        puzzle.little_boxes[78].pretty_print(), puzzle.little_boxes[79].pretty_print(), puzzle.little_boxes[80].pretty_print(),
        ))
    },
    None => {
      HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("It failed! <a href=\"/\">try again</a>?"))
    }
  }
  
}
