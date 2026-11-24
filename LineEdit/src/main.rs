slint::slint!
{
    #[edition = "1.19"]
    
    import { Button, VerticalBox, HorizontalBox, LineEdit } from "std-widgets.slint";
        
    export component Principale inherits Window
    {
        width: 400px;
        height: 300px;
        
        out property <string> texte1: "";
        out property <string> texte2: "";
        out property <string> texte3: "";

        callback texte_saisi1(string);
        callback texte_saisi2(string);
        callback texte_saisi3(string);
        
        VerticalBox
        {
            padding-bottom:10px;
            
            LineEdit
            {
                placeholder-text: "Saisissez votre texte";
                input-type: text; // possibilités : text (par défaut), password: n'affiche que des ”*”, number: n'accepte que 0 à 9, decimal: que nombres réels
                text <=> root.texte1;
                font-size: 20px;
                accepted =>
                {
                    root.texte_saisi1(root.texte1);
                }
                edited =>
                {
                    root.texte_saisi1(root.texte1);
                }
            }
            
            LineEdit
            {
                placeholder-text: "Saisissez votre nombre entier";
                input-type: number; // possibilités : text (par défaut), password: n'affiche que des ”*”, number: n'accepte que 0 à 9, decimal: que nombres réels
                text <=> root.texte2;
                font-size: 20px;
                accepted =>
                {
                    root.texte_saisi2(root.texte2);
                }
                edited =>
                {
                    root.texte_saisi2(root.texte2);
                }
            }
            
            LineEdit
            {
                placeholder-text: "Saisissez votre nombre réel";
                input-type: decimal; // possibilités : text (par défaut), password: n'affiche que des ”*”, number: n'accepte que 0 à 9, decimal: que nombres réels
                text <=> root.texte3;
                font-size: 20px;
                accepted =>
                {
                    root.texte_saisi3(root.texte3);
                }
                edited =>
                {
                    root.texte_saisi3(root.texte3);
                }
            }
            
            Button
            {
                text: "Validez votre saisie";
                clicked => 
                {
                    root.texte_saisi1(root.texte1);
                    root.texte_saisi2(root.texte2);
                    root.texte_saisi3(root.texte3);
                }
            }
        }
    }
}

fn main()
{
    let fenetre = Principale::new().unwrap();

    fenetre.on_texte_saisi1 ( | txt | { println!( "Texte saisi : {txt}" ) } );
    fenetre.on_texte_saisi2 ( | txt | { println!( "Nombre entier saisi : {txt}" ) } );
    fenetre.on_texte_saisi3 ( | txt | { println!( "Nombre réel saisi : {txt}" ) } );
    
    fenetre.run().unwrap();
}
