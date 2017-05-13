package com.example.maxtnt.test_android;

import android.graphics.drawable.BitmapDrawable;
import android.os.Bundle;
import android.support.v7.app.AppCompatActivity;
import android.view.View;
import android.widget.Button;
import android.widget.ImageView;
import android.widget.ScrollView;

public class ImageActivity extends AppCompatActivity {

    ScrollView scroll;
    ImageView imageview;
    BitmapDrawable bitmap;
    int index=2;
    int count=1;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.image_view);

        scroll = (ScrollView) findViewById(R.id.scrollView);
        imageview = (ImageView) findViewById(R.id.imageView);
        Button change_image = (Button) findViewById(R.id.change_image);

        scroll.setHorizontalScrollBarEnabled(true);
        change_image.setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                setCurrentImage(getResources().getIdentifier("test" + count,"drawable",getPackageName()));
                if(index<=count){
                    count=1;
                }else{
                    count+=1;
                }
            }
        });
    }
    private void setCurrentImage(int id){
        bitmap = (BitmapDrawable) getResources().getDrawable(id,null);
        imageview.setImageDrawable(bitmap);
        imageview.getLayoutParams().width = bitmap.getIntrinsicWidth();
        imageview.getLayoutParams().height = bitmap.getIntrinsicHeight();

    }
}