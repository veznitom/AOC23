let read_file filename = 
  let input_channel = open_in filename in
  let rec read_lines lines =
    try
      let line = input_line input_channel in
      read_lines (line :: lines)
    with End_of_file ->
      close_in input_channel;
      List.rev lines in
  read_lines []

let rec sum = function
  | [] -> 0
  | h :: t -> h + sum t

let () =  
  let file = Sys.argv.(1) in
  let lines = read_file file in
  let ints = List.map (
    fun line -> 
      let seq = String.to_seq line in 
      Seq.filter (fun c -> 48 <= Char.code c && Char.code c <= 57) seq |> String.of_seq) lines in
  let short = List.map(
    fun line -> int_of_string((String.sub line 0 1) ^  (String.sub line (String.length line - 1) 1))
  ) ints in
  let s = sum short in
  Printf.printf "%d\n" s;
  ()