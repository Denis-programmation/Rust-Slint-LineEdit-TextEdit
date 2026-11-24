slint::slint!
{
    #[edition = "1.19"]
    
    import { Button, VerticalBox, HorizontalBox, TextEdit } from "std-widgets.slint";
        
    export component Principale inherits Window
    {
        width: 400px;
        height: 400px;
        
        out property <string> texte: "";

        callback texte_saisi(string);
        
        VerticalBox
        {
            padding-bottom:10px;
            
            TextEdit
            {
                placeholder-text: "Saisissez votre texte";
                text <=> root.texte;
                vertical-stretch: 1;
                font-size: 20px;
                edited =>
                {
                    root.texte_saisi(root.texte);
                }
            }
            
            Button
            {
                text: "Valider texte";
                clicked => 
                {
                    root.texte_saisi(root.texte); 
                }
            }
        }
    }
}

fn main()
{
    let fenetre = Principale::new().unwrap();

    fenetre.on_texte_saisi ( | t | { println!( "Texte reçu : {t}" ) } );
    
    fenetre.run().unwrap();
}
